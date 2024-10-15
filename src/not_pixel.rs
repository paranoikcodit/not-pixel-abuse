use std::{
    borrow::BorrowMut,
    fmt::Debug,
    sync::{Arc, Mutex},
};

use crate::{
    telegram::client::Client,
    types::{ClaimResponse, GetMeResponse, GetMiningStatusResponse, PrintResponse, Response},
    utils::{convert_xy_to_pixel_id, generate_default_headers},
};
use anyhow::anyhow;
use reqwest::{Client as ReqwestClient, IntoUrl, Proxy, RequestBuilder};
use serde_json::{json, Value};
use tracing::{info, instrument};

#[derive(Clone)]
pub struct NotPixel {
    client: Arc<Mutex<ReqwestClient>>,
    telegram: Client,
    user_agent: String,
    ref_id: Option<String>,
    proxy: Option<String>,
}

impl NotPixel {
    #[instrument(err, level = "debug", name = "request", skip(self))]
    pub async fn request<T: serde::de::DeserializeOwned + Debug>(
        &self,
        request: RequestBuilder,
    ) -> anyhow::Result<T> {
        let response = request.send().await?;

        match response.error_for_status() {
            Ok(response) => {
                info!(name: "request", "successfully sended request without any error: {} - {}", response.url().to_string(), response.status());

                let response = response.json::<Response<T>>().await?;

                match response {
                    Response::Data(data) => {
                        info!(name: "response", "response successfully received: {:?}", data);

                        Ok(data)
                    }
                    Response::Error(error) => Err(anyhow!(
                        "[{}] Game endpoint error: {}",
                        error.code,
                        error.error
                    )),
                }
            }
            Err(status) => {
                info!("maybe session token is ended - recreating session");

                let new_client = Self::create_client(
                    self.telegram.clone(),
                    self.user_agent.clone(),
                    self.proxy.clone(),
                    self.ref_id.clone(),
                )
                .await?;

                let mut client = self.client.lock().unwrap();
                *client = new_client;

                Err(anyhow!(
                    "[{}] Status code error: {}",
                    status.status().unwrap(),
                    status.url().unwrap(),
                ))
            }
        }
    }

    async fn create_client<T: ToString>(
        telegram: Client,
        user_agent: String,
        proxy: Option<T>,
        ref_id: Option<String>,
    ) -> anyhow::Result<ReqwestClient> {
        let init_data = telegram
            .get_init_data("notpixel".to_string(), "app".to_string(), ref_id)
            .await?;

        let mut client = ReqwestClient::builder()
            .default_headers(generate_default_headers(init_data.to_string(), user_agent));

        if let Some(proxy) = proxy {
            client = client.proxy(Proxy::all(proxy.to_string()).unwrap());
        }

        Ok(client.build()?)
    }

    pub fn post<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        let client = self.client.lock().unwrap();

        client.post(url)
    }

    pub fn get<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        let client = self.client.lock().unwrap();

        client.get(url)
    }

    pub fn delete<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        let client = self.client.lock().unwrap();

        client.delete(url)
    }

    pub fn patch<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        let client = self.client.lock().unwrap();

        client.patch(url)
    }

    pub fn put<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        let client = self.client.lock().unwrap();

        client.put(url)
    }
}

impl NotPixel {
    // TODO: добавить завершение тасков

    pub async fn new(
        telegram: Client,
        user_agent: String,
        proxy: Option<String>,
        ref_id: Option<String>,
    ) -> anyhow::Result<Self> {
        let client = Self::create_client(
            telegram.clone(),
            user_agent.clone(),
            proxy.clone(),
            ref_id.clone(),
        )
        .await?;

        let self_ = Self {
            client: Arc::new(Mutex::new(client)),
            telegram,
            ref_id,
            proxy,
            user_agent,
        };

        self_.get_me().await?;

        Ok(self_)
    }

    pub fn telegram(&self) -> Client {
        self.telegram.clone()
    }

    pub async fn get_me(&self) -> anyhow::Result<GetMeResponse> {
        self.request(self.get("https://notpx.app/api/v1/users/me"))
            .await
    }

    pub async fn get_mining_status(&self) -> anyhow::Result<GetMiningStatusResponse> {
        // TODO: Метрики

        self.request(self.get("https://notpx.app/api/v1/mining/status"))
            .await
    }

    pub async fn claim(&self) -> anyhow::Result<ClaimResponse> {
        // TODO: Метрики
        self.request(self.get("https://notpx.app/api/v1/mining/claim"))
            .await
    }

    pub async fn paint(&self, x: i32, y: i32, color: String) -> anyhow::Result<PrintResponse> {
        // TODO: Метрики

        let pixel_id = convert_xy_to_pixel_id(x, y);

        self.request(
            self.post("https://notpx.app/api/v1/repaint/start")
                .json(&json!({"newColor": color, "pixelId": pixel_id})),
        )
        .await
    }

    pub async fn solve_task(&self, name: String) -> anyhow::Result<bool> {
        self.request(self.get(format!("https://notpx.app/api/v1/mining/task/check/{name}")))
            .await
            .map(|e: Value| e[name].as_bool().unwrap())
    }
}
