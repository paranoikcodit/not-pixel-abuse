use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref TASKS: Vec<String> = vec![
        "invite1fren".to_string(),
        "invite3frens".to_string(),
        "jettonTask".to_string(),
        "paint20pixels".to_string(),
        "joinSquad".to_string(),
        "premium".to_string(),
        "leagueBonusSilver".to_string(),
        "leagueBonusGold".to_string(),
        "leagueBonusPlatinum".to_string(),
        "x:notpixel".to_string(),
        "x:notcoin".to_string(),
        "channel:notpixel_channel".to_string(),
        "channel:notcoin".to_string(),
        "boostChannelNotPixel".to_string(),
        "makePixelAvatar".to_string(),
        "openLeague".to_string(),
        "spendStars".to_string()
    ];

    pub static ref UPGRADES: HashMap<String, Vec<i32>> = {
        let mut upgrades: HashMap<String, Vec<i32>> = HashMap::new();

        // UpgradeRepaint
        upgrades.insert("paint_reward".to_string(), vec![
            5,   // Level 2 Price
            100, // Level 3 Price
            200, // Level 4 Price
            300, // Level 5 Price
            500, // Level 6 Price
            600, // Level 7 Price
        ]);

        // UpgradeChargeRestoration
        upgrades.insert("recharging_speed".to_string(), vec![
            5,    // Level 2 Price
            100,  // Level 3 Price
            200,  // Level 4 Price
            300,  // Level 5 Price
            400,  // Level 6 Price
            500,  // Level 7 Price
            600,  // Level 8 Price
            700,  // Level 9 Price
            800,  // Level 10 Price
            900,  // Level 11 Price
        ]);

        // UpgradeChargeCount
        upgrades.insert("energy_limit".to_string(), vec![
            5,   // Level 2 Price
            100, // Level 3 Price
            200, // Level 4 Price
            300, // Level 5 Price
            400, // Level 6 Price
            10,  // Level 7 Price
        ]);

        upgrades.clone()
    };
}
