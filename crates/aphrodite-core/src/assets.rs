//! Phase 6 — asset management (ADR 0004).
//!
//! Aphrodite v0.3 does not generate images. What it DOES do:
//!   - Wiki entries auto-fetched via `--auto-fetch` carry `og_image` URLs
//!     that the create flow can pull into `<project>/.aphrodite/assets/refs/`
//!     so the design phase has real reference images on disk, not URLs.
//!   - Generated composition / hero produce inline SVGs but may in the future
//!     emit references to png/jpg assets the user supplies.
//!
//! This module defines the directory convention and a content-hash dedupe
//! helper. Concrete fetching lives in the generator crate (HTTP territory).
//!
//! Layout:
//!   <project>/.aphrodite/assets/
//!     refs/        — reference images fetched from wiki entries
//!     uploads/     — user-supplied images (out of scope to manage by Aphrodite;
//!                    convention only)
//!     icons/       — inline-SVG sources extracted from composition (future)
//!
//! Files are content-addressed: `<sha256-prefix>.<ext>`. A second run that
//! sees identical bytes will not write a duplicate.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const HASH_PREFIX_LEN: usize = 16;

pub fn assets_root(project_root: &Path) -> PathBuf {
    let mut p = project_root.to_path_buf();
    p.push(".aphrodite");
    p.push("assets");
    let _ = fs::create_dir_all(&p);
    p
}

pub fn refs_dir(project_root: &Path) -> PathBuf {
    let mut p = assets_root(project_root);
    p.push("refs");
    let _ = fs::create_dir_all(&p);
    p
}

pub fn uploads_dir(project_root: &Path) -> PathBuf {
    let mut p = assets_root(project_root);
    p.push("uploads");
    let _ = fs::create_dir_all(&p);
    p
}

pub fn icons_dir(project_root: &Path) -> PathBuf {
    let mut p = assets_root(project_root);
    p.push("icons");
    let _ = fs::create_dir_all(&p);
    p
}

/// 16-hex-char content prefix (≈64-bit collision space — plenty for a
/// per-project cache, no security claim).
pub fn content_prefix(bytes: &[u8]) -> String {
    // We avoid a sha2 dep by using a small FNV-1a 128 implementation —
    // good-enough cryptographic-non-claim hashing for a content cache.
    let mut h1: u64 = 0xcbf29ce484222325;
    let mut h2: u64 = 0x84222325cbf29ce4;
    for &b in bytes {
        h1 ^= b as u64;
        h1 = h1.wrapping_mul(0x100000001b3);
        h2 ^= (b.rotate_left(3)) as u64;
        h2 = h2.wrapping_mul(0xc70f6907cbf29ce4);
    }
    format!("{:016x}", h1 ^ h2)[..HASH_PREFIX_LEN].to_string()
}

/// Write `bytes` to `<dir>/<hash-prefix>.<ext>` and return its path.
/// If the target already exists with byte-identical content, return its
/// path without rewriting. If a collision lands on the same name but
/// different content, suffix with -1, -2, … (defensive, almost never).
pub fn dedupe_into(dir: &Path, bytes: &[u8], ext: &str) -> Result<PathBuf, io::Error> {
    fs::create_dir_all(dir)?;
    let prefix = content_prefix(bytes);
    let cleaned_ext = ext.trim_start_matches('.');
    let candidate = dir.join(format!("{prefix}.{cleaned_ext}"));
    if candidate.exists() {
        // Trust content-hash collision is extremely rare; if existing bytes
        // happen to differ (different ext used to be saved at same prefix?),
        // suffix.
        match fs::read(&candidate) {
            Ok(existing) if existing == bytes => return Ok(candidate),
            _ => {
                for i in 1..1000 {
                    let alt = dir.join(format!("{prefix}-{i}.{cleaned_ext}"));
                    if !alt.exists() {
                        fs::write(&alt, bytes)?;
                        return Ok(alt);
                    }
                }
                return Err(io::Error::other("dedupe_into: too many collisions"));
            }
        }
    }
    fs::write(&candidate, bytes)?;
    Ok(candidate)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AssetSummary {
    pub category: String, // "refs" / "uploads" / "icons"
    pub path: String,
    pub size_bytes: u64,
    pub extension: String,
}

pub fn list(project_root: &Path) -> Vec<AssetSummary> {
    let root = assets_root(project_root);
    let mut out = Vec::new();
    for category in ["refs", "uploads", "icons"] {
        let dir = root.join(category);
        if let Ok(entries) = fs::read_dir(&dir) {
            for e in entries.flatten() {
                let p = e.path();
                if !p.is_file() {
                    continue;
                }
                let size = p.metadata().map(|m| m.len()).unwrap_or(0);
                let ext = p
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();
                out.push(AssetSummary {
                    category: category.to_string(),
                    path: p.to_string_lossy().to_string(),
                    size_bytes: size,
                    extension: ext,
                });
            }
        }
    }
    out.sort_by(|a, b| a.path.cmp(&b.path));
    out
}

/// Total bytes across all asset categories.
pub fn total_bytes(project_root: &Path) -> u64 {
    list(project_root).iter().map(|s| s.size_bytes).sum()
}

/// Delete every file under `refs/` and `icons/`. Spares `uploads/` (user-
/// supplied content). Returns the count + total bytes reclaimed.
pub fn clean_generated(project_root: &Path) -> Result<(u64, u64), io::Error> {
    let mut n = 0u64;
    let mut bytes = 0u64;
    for category in ["refs", "icons"] {
        let dir = assets_root(project_root).join(category);
        if let Ok(entries) = fs::read_dir(&dir) {
            for e in entries.flatten() {
                let p = e.path();
                if !p.is_file() {
                    continue;
                }
                let size = p.metadata().map(|m| m.len()).unwrap_or(0);
                fs::remove_file(&p)?;
                n += 1;
                bytes += size;
            }
        }
    }
    Ok((n, bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn content_prefix_stable_and_distinguishing() {
        let a = content_prefix(b"hello world");
        let b = content_prefix(b"hello world");
        let c = content_prefix(b"hello world!");
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_eq!(a.len(), HASH_PREFIX_LEN);
    }

    #[test]
    fn dedupe_skips_when_identical_bytes_already_present() {
        let td = tempdir().unwrap();
        let p1 = dedupe_into(td.path(), b"same payload", "txt").unwrap();
        let p2 = dedupe_into(td.path(), b"same payload", "txt").unwrap();
        assert_eq!(p1, p2);
        let entries: Vec<_> = fs::read_dir(td.path()).unwrap().collect();
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn dedupe_keeps_different_bytes_separate() {
        let td = tempdir().unwrap();
        let _ = dedupe_into(td.path(), b"payload one", "png").unwrap();
        let _ = dedupe_into(td.path(), b"payload two", "png").unwrap();
        let entries: Vec<_> = fs::read_dir(td.path()).unwrap().collect();
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn list_and_total_bytes_round_trip() {
        let td = tempdir().unwrap();
        let project = td.path();
        let refs = refs_dir(project);
        fs::write(refs.join("aa.png"), b"hello").unwrap();
        fs::write(refs.join("bb.jpg"), b"world!").unwrap();
        let assets = list(project);
        assert_eq!(assets.len(), 2);
        assert!(assets.iter().all(|s| s.category == "refs"));
        assert_eq!(total_bytes(project), 11);
    }

    #[test]
    fn clean_generated_spares_uploads() {
        let td = tempdir().unwrap();
        let p = td.path();
        fs::write(refs_dir(p).join("ref.png"), b"ref").unwrap();
        fs::write(icons_dir(p).join("icon.svg"), b"icon").unwrap();
        fs::write(uploads_dir(p).join("user.jpg"), b"user upload").unwrap();
        let (n, _) = clean_generated(p).unwrap();
        assert_eq!(n, 2);
        assert!(uploads_dir(p).join("user.jpg").exists());
        assert!(!refs_dir(p).join("ref.png").exists());
    }
}
