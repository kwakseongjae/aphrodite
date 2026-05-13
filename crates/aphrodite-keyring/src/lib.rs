//! Keyring abstraction. Single secret-touching code path in the workspace.
//!
//! macOS path uses the `security` CLI (subprocess) because the Rust `keyring`
//! crate has a known same-process set→read quirk on Sequoia+ where `set_password`
//! succeeds but an immediate `get_password` through a fresh Entry returns
//! NoEntry. Subprocess is slower but reliable.
//!
//! Linux + Windows use the `keyring` crate (libsecret / Credential Manager).

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeyringError {
    #[error("backend error: {0}")]
    Backend(String),
    #[error("no key stored for provider {0}")]
    NotFound(String),
}

impl From<keyring::Error> for KeyringError {
    fn from(e: keyring::Error) -> Self {
        match e {
            keyring::Error::NoEntry => KeyringError::NotFound(String::new()),
            other => KeyringError::Backend(other.to_string()),
        }
    }
}

const SERVICE: &str = "aphrodite";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Credential {
    ApiKey { value: String },
    OAuth { access: String, refresh: Option<String>, expires_at: Option<i64> },
}

fn account(provider: &str) -> String {
    format!("provider:{provider}")
}

pub fn store(provider: &str, secret: &str) -> Result<(), KeyringError> {
    #[cfg(target_os = "macos")]
    {
        return macos::store(provider, secret);
    }
    #[cfg(not(target_os = "macos"))]
    {
        let entry = keyring::Entry::new(SERVICE, &account(provider))?;
        entry.set_password(secret)?;
        let fresh = keyring::Entry::new(SERVICE, &account(provider))?;
        let back = fresh.get_password()?;
        if back != secret {
            return Err(KeyringError::Backend("readback mismatch".into()));
        }
        Ok(())
    }
}

pub fn fetch(provider: &str) -> Result<String, KeyringError> {
    #[cfg(target_os = "macos")]
    {
        return macos::fetch(provider);
    }
    #[cfg(not(target_os = "macos"))]
    {
        let entry = keyring::Entry::new(SERVICE, &account(provider))?;
        entry.get_password().map_err(|e| match e {
            keyring::Error::NoEntry => KeyringError::NotFound(provider.into()),
            other => KeyringError::Backend(other.to_string()),
        })
    }
}

pub fn delete(provider: &str) -> Result<(), KeyringError> {
    #[cfg(target_os = "macos")]
    {
        return macos::delete(provider);
    }
    #[cfg(not(target_os = "macos"))]
    {
        let entry = keyring::Entry::new(SERVICE, &account(provider))?;
        entry.delete_credential()?;
        Ok(())
    }
}

pub fn store_oauth(
    provider: &str,
    access: &str,
    refresh: Option<&str>,
    expires_at: Option<i64>,
) -> Result<(), KeyringError> {
    let _ = expires_at; // v0.2 will write the timestamp slot
    let _ = refresh;
    let _ = provider;
    let _ = access;
    Err(KeyringError::Backend("OAuth slots land in v0.2".into()))
}

pub fn fetch_credential(provider: &str) -> Result<Credential, KeyringError> {
    fetch(provider).map(|value| Credential::ApiKey { value })
}

#[cfg(target_os = "macos")]
mod macos {
    //! Shells out to `/usr/bin/security`. This is the same tool Apple ships
    //! and the same one `aphrodite doctor` uses for the native verification
    //! check — so what `init` writes is exactly what `doctor` reads.

    use super::{account, KeyringError, SERVICE};
    use std::process::Command;

    fn security() -> Command {
        Command::new("/usr/bin/security")
    }

    pub fn store(provider: &str, secret: &str) -> Result<(), KeyringError> {
        // -U updates if exists; -s service; -a account; -w write password (last arg).
        // We pass the secret via stdin alternative? `security` requires -w VALUE on
        // argv unfortunately. argv is visible briefly to root in `ps`. Document
        // accept-this-tradeoff; alternative is keychain access via API.
        let out = security()
            .arg("add-generic-password")
            .arg("-U")
            .arg("-s")
            .arg(SERVICE)
            .arg("-a")
            .arg(account(provider))
            .arg("-w")
            .arg(secret)
            .output()
            .map_err(|e| KeyringError::Backend(format!("spawn security: {e}")))?;
        if !out.status.success() {
            let err = String::from_utf8_lossy(&out.stderr).to_string();
            return Err(KeyringError::Backend(format!(
                "security add-generic-password exit {}: {}",
                out.status.code().unwrap_or(-1),
                err.trim()
            )));
        }
        // Immediate readback through the same CLI.
        let back = fetch(provider)?;
        if back != secret {
            return Err(KeyringError::Backend(
                "readback mismatch after store".into(),
            ));
        }
        Ok(())
    }

    pub fn fetch(provider: &str) -> Result<String, KeyringError> {
        let out = security()
            .arg("find-generic-password")
            .arg("-s")
            .arg(SERVICE)
            .arg("-a")
            .arg(account(provider))
            .arg("-w")
            .output()
            .map_err(|e| KeyringError::Backend(format!("spawn security: {e}")))?;
        if !out.status.success() {
            return Err(KeyringError::NotFound(provider.to_string()));
        }
        let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if s.is_empty() {
            return Err(KeyringError::NotFound(provider.to_string()));
        }
        Ok(s)
    }

    pub fn delete(provider: &str) -> Result<(), KeyringError> {
        let _ = security()
            .arg("delete-generic-password")
            .arg("-s")
            .arg(SERVICE)
            .arg("-a")
            .arg(account(provider))
            .output();
        Ok(())
    }
}
