use rand::RngCore;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};


pub fn generate_key() -> String {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(&bytes)
}