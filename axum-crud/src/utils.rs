use base58::ToBase58;
use uuid::Uuid;

use crate::error::Result;

pub fn b32_hex() -> Result<String> {
    let uuid = Uuid::now_v7();

    Ok(data_encoding::BASE32HEX_NOPAD.encode(uuid.as_bytes()))
}

pub fn b64() -> Result<String> {
    let uuid = Uuid::now_v7();
    Ok(data_encoding::BASE64.encode(uuid.as_bytes()))
}

pub fn b64u() -> Result<String> {
    let uuid = Uuid::now_v7();

    Ok(data_encoding::BASE64_NOPAD.encode(uuid.as_bytes()))
}

pub fn b58() -> Result<String> {
    let uuid = Uuid::now_v7();
    Ok(uuid.as_bytes().to_base58())
}
