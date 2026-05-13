//! Keyring abstraction. The *only* place in the workspace that touches
//! credentials. Every other crate goes through this API.
//!
//! Service name: `aphrodite`. Account: `provider:<ProviderId>`.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeyringError {
    #[error("backend error: {0}")]
    Backend(#[from] keyring::Error),
    #[error("no key stored for provider {0}")]
    NotFound(String),
}

const SERVICE: &str = "aphrodite";

pub fn store(provider: &str, secret: &str) -> Result<(), KeyringError> {
    let entry = keyring::Entry::new(SERVICE, &format!("provider:{provider}"))?;
    entry.set_password(secret)?;
    Ok(())
}

pub fn fetch(provider: &str) -> Result<String, KeyringError> {
    let entry = keyring::Entry::new(SERVICE, &format!("provider:{provider}"))?;
    entry.get_password().map_err(|e| match e {
        keyring::Error::NoEntry => KeyringError::NotFound(provider.into()),
        other => other.into(),
    })
}

pub fn delete(provider: &str) -> Result<(), KeyringError> {
    let entry = keyring::Entry::new(SERVICE, &format!("provider:{provider}"))?;
    entry.delete_credential()?;
    Ok(())
}
