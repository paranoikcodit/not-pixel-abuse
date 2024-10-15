use agent::generate_random_user_agent;
use config::Config;
use telegram::{client::Client, sessions::read_sessions};
use tracing::{error, info, instrument, level_filters::LevelFilter};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use worker::create_worker;

mod agent;
mod config;
mod constants;
mod not_pixel;
mod telegram;
mod types;
mod utils;
mod worker;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TODO: сделать добавление сессий и их оптимальную загрузку в память
    // TODO: обработка сессий вместе с аккаунтами
    // TODO: разбить все на воркеры

    let registry = tracing_subscriber::registry().with(fmt::layer()).with(
        EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy(),
    );

    #[cfg(feature = "tui")]
    {
        registry.with(tui_logger::tracing_subscriber_layer());
        tui_logger::init_logger(log::LevelFilter::Trace)?;
    };

    registry.init();

    let config = tokio::fs::read_to_string("config.toml").await?;
    let config = toml::from_str::<Config>(&config)?;

    let proxies = tokio::fs::read_to_string(config.proxies_path.clone())
        .await?
        .trim()
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();

    let sessions = read_sessions(
        config.sessions_path.clone(),
        proxies,
        config.api_id.clone(),
        config.api_hash.clone(),
    )?;

    info!("all sessions is loaded: {}", sessions.len());

    info!("start execution");

    let handles = sessions.iter().cloned().map(|session| {
        tokio::spawn(create_worker(
            config.clone(),
            session.0,
            session.1,
            session.2,
        ))
    });

    futures::future::join_all(handles).await;

    Ok(())
}
