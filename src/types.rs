use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Error {
    pub error: String,
    pub code: i16,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Response<T> {
    Error(Error),
    Data(T),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMeResponse {
    pub id: i64,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub balance: i64,
    pub repaints: i64,
    pub score: Value,
    pub language: String,
    #[serde(rename = "isPremium")]
    pub is_premium: Option<bool>,
    pub friends: i64,
    pub intro: bool,
    #[serde(rename = "userPic")]
    pub user_pic: String,
    pub league: String,
    #[serde(rename = "templateId")]
    pub template_id: i64,
    pub squad: Option<Squad>,
    pub goods: Value,
    #[serde(rename = "refLimit")]
    pub ref_limit: i64,
    #[serde(rename = "websocketToken")]
    pub websocket_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Squad {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub logo: Option<String>,
    #[serde(rename = "templateX")]
    pub template_x: Option<i64>,
    #[serde(rename = "templateY")]
    pub template_y: Option<i64>,
    pub players: Value,
    #[serde(rename = "totalBalance")]
    pub total_balance: Value,
    #[serde(rename = "totalRepaints")]
    pub total_repaints: Value,
    #[serde(rename = "scoredRepaints")]
    pub scored_repaints: Value,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct PrintResponse {
    balance: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMiningStatusResponse {
    pub coins: f64,
    #[serde(rename = "speedPerSecond")]
    pub speed_per_second: f64,
    #[serde(rename = "fromStart")]
    pub from_start: i64,
    #[serde(rename = "fromUpdate")]
    pub from_update: i64,
    #[serde(rename = "maxMiningTime")]
    pub max_mining_time: i64,
    pub claimed: i64,
    pub boosts: Value,
    #[serde(rename = "repaintsTotal")]
    pub repaints_total: i64,
    #[serde(rename = "userBalance")]
    pub user_balance: f64,
    pub activated: bool,
    pub league: String,
    pub charges: i64,
    #[serde(rename = "maxCharges")]
    pub max_charges: i64,
    #[serde(rename = "reChargeTimer")]
    pub re_charge_timer: i64,
    #[serde(rename = "reChargeSpeed")]
    pub re_charge_speed: i64,
    pub goods: Goods,
    pub tasks: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Boosts {
    #[serde(rename = "paintReward")]
    pub paint_reward: i64,
    #[serde(rename = "reChargeSpeed")]
    pub re_charge_speed: i64,
    #[serde(rename = "energyLimit")]
    pub energy_limit: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Goods {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tasks {
    #[serde(rename = "boostChannelNotPixel")]
    pub boost_channel_not_pixel: bool,
    #[serde(rename = "joinSquad")]
    pub join_squad: bool,
    #[serde(rename = "makePixelAvatar")]
    pub make_pixel_avatar: bool,
    pub premium: bool,
    #[serde(rename = "x:notcoin")]
    pub x_notcoin: bool,
    #[serde(rename = "x:notpixel")]
    pub x_notpixel: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClaimResponse {
    pub coins: i64,
    #[serde(rename = "speedPerSecond")]
    pub speed_per_second: f64,
    #[serde(rename = "fromStart")]
    pub from_start: i64,
    #[serde(rename = "fromUpdate")]
    pub from_update: i64,
    #[serde(rename = "maxMiningTime")]
    pub max_mining_time: i64,
    pub claimed: f64,
    pub boosts: Boosts,
    #[serde(rename = "repaintsTotal")]
    pub repaints_total: i64,
    #[serde(rename = "userBalance")]
    pub user_balance: i64,
    pub activated: bool,
    pub league: String,
    pub charges: i64,
    #[serde(rename = "maxCharges")]
    pub max_charges: i64,
    #[serde(rename = "reChargeTimer")]
    pub re_charge_timer: i64,
    #[serde(rename = "reChargeSpeed")]
    pub re_charge_speed: i64,
    pub goods: Value,
    pub tasks: Value,
}
