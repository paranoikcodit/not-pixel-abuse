use anyhow::Ok;
use grammers_tl_types::{enums::Chat, types::InputChannel};

#[derive(Clone)]
pub struct Client {
    pub inner: grammers_client::Client,
}

impl From<grammers_client::Client> for Client {
    fn from(inner: grammers_client::Client) -> Self {
        Self { inner }
    }
}

impl Client {
    pub fn new(inner: grammers_client::Client) -> Self {
        Self { inner }
    }

    pub async fn get_channel(&self, username: String) -> anyhow::Result<bool> {
        let channel_id = self.inner.resolve_username(&username).await?.unwrap();

        let access_hash = channel_id.pack().access_hash.unwrap();
        let id = channel_id.id();

        let result = self
            .inner
            .invoke(&grammers_tl_types::functions::channels::GetChannels {
                id: vec![InputChannel {
                    access_hash: access_hash,
                    channel_id: id,
                }
                .into()],
            })
            .await?
            .chats();

        let result = result.first();

        if let Some(Chat::Channel(channel)) = result.clone() {
            return Ok(channel.left);
        }

        Ok(false)
    }

    pub async fn join_channel(&self, username: String) -> anyhow::Result<()> {
        self.inner
            .join_chat(self.inner.resolve_username(&username).await?.unwrap())
            .await?;

        Ok(())
    }

    pub async fn get_init_data(
        &self,
        username: String,
        short_name: String,
        start_param: Option<String>,
    ) -> anyhow::Result<String> {
        let chat = self.inner.resolve_username(&username).await?.unwrap();

        let req = grammers_tl_types::functions::messages::RequestAppWebView {
            start_param,
            platform: "android".to_string(),
            write_allowed: true,
            peer: self.inner.get_me().await?.pack().to_input_peer(),
            app: grammers_tl_types::enums::InputBotApp::ShortName(
                grammers_tl_types::types::InputBotAppShortName {
                    bot_id: chat.pack().try_to_input_user().unwrap(),
                    short_name,
                },
            ),
            theme_params: None,
            compact: false,
        };

        let response = self.inner.invoke(&req).await?;

        let init_data = match response {
            grammers_tl_types::enums::WebViewResult::Url(ref url) => {
                urlencoding::decode(url.url.split("#tgWebAppData=").last().unwrap())?.to_string()
            }
        };

        Ok(init_data
            .split("&tgWebAppVersion=")
            .next()
            .unwrap()
            .to_string())
    }
}
