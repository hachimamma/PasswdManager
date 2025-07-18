//sorry but this is not much readable, do not judge

use ring::{
    aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM},
    pbkdf2,
};
use arrayref::array_ref;
use anyhow::anyhow;

pub const SALT_LEN: usize  = 16;
pub const NONCE_LEN: usize = 12;
const PBKDF2_ITER: u32     = 100_000;

pub fn derive_key(master: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        std::num::NonZeroU32::new(PBKDF2_ITER).unwrap(),
        salt,
        master.as_bytes(),
        &mut key,
    );
    key
}

pub fn encrypt(key: &[u8; 32], nonce: &[u8; NONCE_LEN], mut data: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let key = LessSafeKey::new(UnboundKey::new(&AES_256_GCM, key).map_err(|e| anyhow!("ring: {e:?}"))?);

    key.seal_in_place_append_tag(
        Nonce::assume_unique_for_key(*array_ref!(nonce, 0, 12)),
        Aad::empty(),
        &mut data,
    )
    .map_err(|e| anyhow!("ring: {e:?}"))?;

    Ok(data)
}

pub fn decrypt(key: &[u8; 32], nonce: &[u8; NONCE_LEN], mut ct: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let key = LessSafeKey::new(UnboundKey::new(&AES_256_GCM, key).map_err(|e| anyhow!("ring: {e:?}"))?);

    let pt = key
        .open_in_place(
            Nonce::assume_unique_for_key(*array_ref!(nonce, 0, 12)),
            Aad::empty(),
            &mut ct,
        )
        .map_err(|e| anyhow!("ring: {e:?}"))?;

    Ok(pt.to_vec())
}
