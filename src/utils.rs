use std::{collections::BTreeSet, num::ParseIntError};

use reqwest::header::{HeaderMap, AUTHORIZATION, USER_AGENT};

pub type PixelId = i32;

pub fn convert_xy_to_pixel_id(x: i32, y: i32) -> PixelId {
    x + y * 1000 + 1
}

pub fn generate_default_headers(init_data: String, user_agent: String) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        AUTHORIZATION,
        format!("initData {init_data}").parse().unwrap(),
    );
    headers.insert(USER_AGENT, user_agent.parse().unwrap());

    headers
}

pub fn get_hashes(path: impl AsRef<str>) -> Vec<(String, Vec<u8>)> {
    let path = path.as_ref();

    merkle_hash::MerkleTree::builder(path)
        .algorithm(merkle_hash::Algorithm::Blake3)
        .hash_names(true)
        .build()
        .unwrap()
        .into_iter()
        .filter_map(|item| {
            let path_ = item.path.relative.to_string();
            let parts = path_.split("/").collect::<Vec<_>>();

            if parts.len() == 1 && !parts[0].is_empty() {
                Some((format!("{}/{}", path, path_), item.hash))
            } else {
                None
            }
        })
        .collect()
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}
