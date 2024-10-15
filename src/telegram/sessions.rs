use std::{fmt::Debug, path::Path};

use anyhow::anyhow;
use grammers_client::{Config, InitParams};
use grammers_tl_types::Serializable;
use merkle_hash::Encodable;
use rustytele::{session::Session, PyrogramSession, TelegramDesktop, TelethonSession};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{error, info, instrument, span, Instrument, Level};

use crate::utils::{decode_hex, get_hashes};

#[instrument(err)]
pub async fn read_session<T>(
    session: T,
    api_id: Option<i32>,
    api_hash: Option<String>,
) -> anyhow::Result<(grammers_client::session::Session, i32, String)>
where
    T: AsRef<Path> + Debug,
{
    let path = session.as_ref().to_str().unwrap();

    let (api_id, api_hash) = api_id
        .zip(api_hash)
        .ok_or(anyhow!("api_id or api_hash is not provided in config"))?;

    let (data, api_id, api_hash) = if PyrogramSession::validate(path)? {
        (PyrogramSession::open(path)?.serialize(), api_id, api_hash)
    } else if TelethonSession::validate(path)? {
        (TelethonSession::open(path)?.serialize(), api_id, api_hash)
    } else if TelegramDesktop::validate(path)? {
        (
            TelegramDesktop::open(path)?.serialize(),
            2040,
            String::from("b18441a1ff607e10a989891a5462e627"),
        )
    } else {
        return Err(anyhow!("Failed to read session: {path}"));
    };

    let session = grammers_client::session::Session::load(&data)?;

    Ok((session, api_id, api_hash))
}

#[instrument(err, skip(proxies, api_id, api_hash, path))]
pub fn read_sessions<T>(
    path: T,
    proxies: Vec<String>,
    api_id: Option<i32>,
    api_hash: Option<String>,
) -> anyhow::Result<Vec<(super::client::Client, String, String)>>
where
    T: AsRef<Path> + Debug + Clone,
{
    let cached_sessions = sled::open("cached_sessions")?;
    let tree = get_hashes(path.as_ref().to_str().unwrap());

    if tree.len() > proxies.len() {
        return Err(anyhow!("Proxies length should be equal sessions length"));
    }

    let sessions = tree
        .iter()
        .zip(proxies.iter())
        .map(|((session, hash), proxy)| {
            let api_hash = api_hash.clone();
            let value = cached_sessions.clone();

            let span = span!(Level::ERROR, "resolve session");

            async move {
                let (session, user_agent, api_id, api_hash) =
                    if let Ok(Some(session)) = value.get(hash.clone()) {
                        let mut session = &session[..];

                        let mut api_hash: [u8; 16] = [0u8; 16];
                        // let mut user_agent = []
                        let mut session_data = Vec::new();

                        let api_id = session.read_i32().await?;
                        session.read(&mut api_hash).await?;

                        let len = session.read_i32().await?;
                        let mut user_agent = vec![0u8; len as usize];
                        session.read_exact(&mut user_agent).await?;

                        session.read_to_end(&mut session_data).await?;

                        let api_hash = api_hash.to_vec().to_hex_string();

                        let session = match grammers_client::session::Session::load(&session_data) {
                            Ok(data) => {
                                info!("Success restoring session from cache");

                                data
                            }
                            Err(error) => {
                                error!("Failed to restore session from cache: {}", error);

                                return Err(error.into());
                            }
                        };

                        (
                            session,
                            String::from_utf8_lossy(&user_agent).to_string(),
                            api_id,
                            api_hash,
                        )
                    } else {
                        let (session, api_id, api_hash) =
                            read_session(session, api_id, api_hash).await?;

                        let hash_ = decode_hex(&api_hash)?;
                        let user_agent =
                            crate::agent::generate_random_user_agent("android", "chrome").unwrap();

                        let user_agent_ = user_agent.clone().as_bytes().to_vec();

                        let mut buffer = vec![];

                        buffer.write_i32(api_id).await?;
                        buffer.write(&hash_).await?;
                        buffer.write_i32(user_agent_.len() as i32).await?;
                        buffer.write(&user_agent_).await?;

                        buffer.write(&session.save()).await?;

                        buffer.flush().await?;

                        value.insert(hash, buffer)?;

                        (session, user_agent, api_id, api_hash)
                    };

                Result::<(super::client::Client, String, String), anyhow::Error>::Ok((
                    super::client::Client::from(
                        grammers_client::Client::connect(Config {
                            api_id,
                            api_hash,
                            session,
                            params: InitParams {
                                proxy_url: Some(proxy.clone()),
                                ..Default::default()
                            },
                        })
                        .await?,
                    ),
                    user_agent,
                    proxy.clone(),
                ))
            }
            .instrument(span)
        });

    cached_sessions.flush()?;

    let results = futures::executor::block_on(futures::future::join_all(sessions));
    let mut sessions = vec![];

    for session in results {
        sessions.push(session?);
    }

    Ok(sessions)
}
