use hmac::{Hmac, Mac};
use sha2::Sha256;
use subtle::ConstantTimeEq;

use crate::webhook::{
    HeaderAccess, VerificationError,
    types::{MESSAGE_ID, MESSAGE_SIGNATURE, MESSAGE_TIMESTAMP},
};

const HMAC_PREFIX: &str = "sha256=";

type HmacSha256 = Hmac<Sha256>;

pub fn verify_event_message<H: HeaderAccess>(
    headers: &H,
    body: &[u8],
    secret: &str,
) -> Result<(), VerificationError> {
    let signature = headers
        .get_header(MESSAGE_SIGNATURE)
        .ok_or(VerificationError::MissingHeader(MESSAGE_SIGNATURE))?;

    let hmac_signature = hmac_signature(headers, body, secret)?;

    if verify_message(&hmac_signature, signature) {
        Ok(())
    } else {
        Err(VerificationError::InvalidSignature)
    }
}

fn get_hmac_message<H: HeaderAccess>(
    headers: &H,
    body: &[u8],
) -> Result<Vec<u8>, VerificationError> {
    let message_id = headers
        .get_header(MESSAGE_ID)
        .ok_or(VerificationError::MissingHeader(MESSAGE_ID))?;

    let message_timestamp = headers
        .get_header(MESSAGE_TIMESTAMP)
        .ok_or(VerificationError::MissingHeader(MESSAGE_TIMESTAMP))?;

    let mut message = Vec::with_capacity(message_id.len() + message_timestamp.len() + body.len());

    message.extend_from_slice(message_id.as_bytes());
    message.extend_from_slice(message_timestamp.as_bytes());
    message.extend_from_slice(body);
    Ok(message)
}

fn get_hmac(secret: &str, message: &[u8]) -> String {
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(message);

    format!(
        "{}{}",
        HMAC_PREFIX,
        hex::encode(mac.finalize().into_bytes())
    )
}

fn hmac_signature<H: HeaderAccess>(
    headers: &H,
    body: &[u8],
    secret: &str,
) -> Result<String, VerificationError> {
    Ok(get_hmac(secret, &get_hmac_message(headers, body)?))
}

fn verify_message(hmac: &str, signature: &str) -> bool {
    if hmac.len() != signature.len() {
        return false;
    }

    hmac.as_bytes().ct_eq(signature.as_bytes()).into()
}
