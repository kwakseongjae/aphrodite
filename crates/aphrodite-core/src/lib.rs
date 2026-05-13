//! aphrodite-core — the shared invariants of the Aphrodite harness.
//!
//! Modules in this crate define the *types* that every binding (CLI, MCP, TS, Python)
//! agrees on. Concrete adapters (LLM providers, file emitters) live in sibling crates.

pub mod config;     // ~/.aphrodite/config.toml — provider preferences
pub mod design;     // DESIGN.md parse / model / serialize
pub mod validator;  // schema + WCAG-AA contrast
pub mod variant;    // light / dark / brand resolution
pub mod taste;      // global ⊕ project taste store (.jsonl + sqlite)
pub mod policy;     // deny-list policy
pub mod invocation; // Invocation type — the contract entry point
pub mod seed;       // Reads .ouroboros/seeds/*.yaml

pub use design::{parse as parse_design, DesignDocument, DesignError, SectionKind};
pub use invocation::{Caller, Invocation, Surface, WriteMode};
pub use policy::Policy;
pub use seed::{load as load_seed, Seed};
pub use taste::{record as record_taste, snapshot_for as taste_snapshot, SignalKind, TasteEvent, TasteSnapshot};
pub use validator::{validate as validate_design, ValidationReport, ValidationViolation, ViolationKind};
pub use variant::{resolve as resolve_variants, Variant, VariantKind};
