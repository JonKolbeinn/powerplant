use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use std::error::Error;

const NONCE_LENGTH: usize = 16;
const NONCE_TAG: &str = "nonce";

#[derive(Serialize, Deserialize, Clone)]
pub struct NostrEvent {
    pub created_at: u64,
    pub kind: u32,
    pub tags: Vec<Vec<String>>,
    pub content: String,
    pub pubkey: String,
}

#[derive(Serialize, Deserialize)]
pub struct PowRequest {
    pub event: NostrEvent,
    pub target_pow: u32,
}

#[derive(Serialize, Deserialize)]
pub struct PowResponse {
    pub event: NostrEvent,
    pub pow: u32,
}

pub fn perform_pow(request: PowRequest) -> Result<PowResponse, Box<dyn Error>> {
    let mut event = request.event;
    let nonce_placeholder = "0".repeat(NONCE_LENGTH);
    let nonce_tag = vec![NONCE_TAG.to_string(), nonce_placeholder.clone()];
    event.tags.push(nonce_tag);
    let nonce_tag_index = event.tags.len() - 1;

    // Serialize the event with the nonce placeholder
    let serialized = serde_json::to_string(&event)?;
    let nonce_search_str = format!("\"{}\":\"{}\"", NONCE_TAG, nonce_placeholder);
    let nonce_position = serialized.find(&nonce_search_str)
        .ok_or("Nonce placeholder not found in the serialized event")?;
    let nonce_value_position = nonce_position + nonce_search_str.find(&nonce_placeholder).unwrap();

    let prefix = &serialized[..nonce_value_position];
    let suffix = &serialized[nonce_value_position + NONCE_LENGTH..];
    let mut nonce = 0u128;

    loop {
        if nonce == u128::MAX {
            return Err("Nonce overflow occurred".into());
        }

        let nonce_hex = format!("{:016x}", nonce);
        let new_serialized = format!("{}{}{}", prefix, nonce_hex, suffix);

        let hash = Sha256::digest(new_serialized.as_bytes());
        let pow = count_leading_zeroes(&hash);

        if pow >= request.target_pow {
            event.tags[nonce_tag_index][1] = nonce_hex;
            return Ok(PowResponse { event, pow });
        }

        nonce += 1;
    }
}

fn count_leading_zeroes(hash: &[u8]) -> u32 {
    let mut count = 0;
    for &byte in hash {
        let zeros = byte.leading_zeros();
        count += zeros;
        if zeros < 8 {
            break;
        }
    }
    count
}
