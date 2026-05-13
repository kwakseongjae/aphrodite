//! Keyring abstraction. The *only* place in the workspace that touches
//! credentials. Every other crate goes through this API.
//!
//! Storage layout (service = `aphrodite`):
//!   * `provider:<id>`               — API key (v0.1 default)
//!   * `provider:<id>:oauth_access`  — OAuth access token (v0.2)
//!   * `provider:<id>:oauth_refresh` — OAuth refresh token (v0.2)
//!   * `provider:<id>:oauth_expiry`  — Unix-ts string for the access token
//!
//! v0.1 only writes the first slot. The other slots are reserved so v0.2
//! OAuth flows can land without touching call sites.

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeyringError {
    #[error("backend error: {0}")]
    Backend(#[from] keyring::Error),
    #[error("no key stored for provider {0}")]
    NotFound(String),
}

const SERVICE: &str = "aphrodite";

/// Credential variants. v0.1 only emits `ApiKey`; the OAuth variant exists so
/// downstream code keeps a stable surface when v0.2 ships.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Credential {
    ApiKey { value: String },
    OAuth { access: String, refresh: Option<String>, expires_at: Option<i64> },
}

/// Write an API key + verify the OS keychain entry round-trips. Returns Ok
/// only when the store *and* a fresh subsequent fetch both succeed.
pub fn store(provider: &str, secret: &str) -> Result<(), KeyringError> {
    let entry = keyring::Entry::new(SERVICE, &format!("provider:{provider}"))?;
    entry.set_password(secret)?;
    // Immediate readback through a *fresh* Entry so we exercise the same
    // permission gate `fetch()` will see. macOS sometimes accepts a write but
    // blocks subsequent reads from a different process or different
    // permission context — we catch that here, not at design-call time.
    let fresh = keyring::Entry::new(SERVICE, &format!("provider:{provider}"))?;
    let back = fresh.get_password()?;
    if back != secret {
        return Err(KeyringError::Backend(keyring::Error::Invalid(
            "keychain readback mismatch".into(),
            "fetched value differs from stored value".into(),
        )));
    }
    Ok(())
}

/// Read the API key from the OS keychain. Returns `NotFound` when no entry
/// exists. Per seed acceptance #9, secrets only live in the keychain — there
/// is no on-disk file fallback for `secret material`.
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

/// v0.2 OAuth write. Lives now so we don't break the public API later.
pub fn store_oauth(
    provider: &str,
    access: &str,
    refresh: Option<&str>,
    expires_at: Option<i64>,
) -> Result<(), KeyringError> {
    keyring::Entry::new(SERVICE, &format!("provider:{provider}:oauth_access"))?
        .set_password(access)?;
    if let Some(r) = refresh {
        keyring::Entry::new(SERVICE, &format!("provider:{provider}:oauth_refresh"))?
            .set_password(r)?;
    }
    if let Some(exp) = expires_at {
        keyring::Entry::new(SERVICE, &format!("provider:{provider}:oauth_expiry"))?
            .set_password(&exp.to_string())?;
    }
    Ok(())
}

/// Best-effort full credential read. v0.1 only sees `ApiKey`. v0.2 will
/// resolve OAuth variants here.
pub fn fetch_credential(provider: &str) -> Result<Credential, KeyringError> {
    match fetch(provider) {
        Ok(value) => Ok(Credential::ApiKey { value }),
        Err(e) => Err(e),
    }
}
