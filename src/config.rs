use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Paint {
    pub delay: u64,
    pub colors: Vec<String>,
    pub coords: Option<((i32, i32), (i32, i32))>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Boosts {
    pub minimal_amount_to_save: i64,
    pub exclude: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Tasks {
    pub needed: Vec<String>,
    pub channel_join: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub time_to_start: (u32, u32),
    pub paint: Option<Paint>,
    pub boosts: Option<Boosts>,
    pub need_claim: Option<bool>,
    pub tasks: Option<Tasks>,
    pub ref_id: Option<String>,
    pub sessions_path: String,
    pub proxies_path: String,
    pub api_id: Option<i32>,
    pub api_hash: Option<String>,
}
