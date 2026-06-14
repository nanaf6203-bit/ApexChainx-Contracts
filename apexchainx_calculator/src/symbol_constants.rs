//! Canonical Symbol constants used across the contract ecosystem.
//!
//! This module centralises all Symbol constants used for storage keys, event
//! names, result status values, payment types, rating tiers, and severity
//! levels. Centralising these constants ensures:
//!
//! 1. **Consistency** — The same Symbol is used everywhere it's referenced
//! 2. **Auditability** — A single location to review all on-chain identifiers
//! 3. **Collision prevention** — All constants are defined together, making
//!    accidental duplicates immediately visible
//!
//! # Symbol Categories
//!
//! | Category | Prefix | Used For |
//! |----------|--------|----------|
//! | Storage Keys | `KEY_*` | On-chain persistent storage |
//! | Event Names | `EVT_*` | Event topic[0] identifiers |
//! | Result Status | `STATUS_*` | SLA outcome symbols |
//! | Payment Type | `PAY_*` | Financial outcome symbols |
//! | Rating Tier | `RATING_*` | Performance rating symbols |
//! | Severity Level | `SEV_*` | SLA severity identifiers |
//!
//! # Constraints
//!
//! All Symbol constants must fit within Soroban's 9-character `symbol_short!`
//! limit. Constants exceeding this length must use `Symbol::new()` instead.

use soroban_sdk::{symbol_short, Symbol};

// ── Storage keys ─────────────────────────────────────────────────────────────
pub const KEY_ADMIN:    Symbol = symbol_short!("ADMIN");
pub const KEY_OPERATOR: Symbol = symbol_short!("OPERATOR");
pub const KEY_CONFIG:   Symbol = symbol_short!("CONFIG");
pub const KEY_PAUSED:   Symbol = symbol_short!("PAUSED");
pub const KEY_STATS:    Symbol = symbol_short!("STATS");
pub const KEY_HISTORY:  Symbol = symbol_short!("HIST");
pub const KEY_VERSION:  Symbol = symbol_short!("VER");

// ── Event names ───────────────────────────────────────────────────────────────
pub const EVT_SLA_CALC:    Symbol = symbol_short!("sla_calc");
pub const EVT_CONFIG_UPD:  Symbol = symbol_short!("cfg_upd");
pub const EVT_PAUSED:      Symbol = symbol_short!("paused");
pub const EVT_UNPAUSED:    Symbol = symbol_short!("unpause");
pub const EVT_OP_SET:      Symbol = symbol_short!("op_set");
pub const EVT_PRUNED:      Symbol = symbol_short!("pruned");
pub const EVT_VERSION:     Symbol = symbol_short!("v1");

// ── Result status ─────────────────────────────────────────────────────────────
pub const STATUS_MET:  Symbol = symbol_short!("met");
pub const STATUS_VIOL: Symbol = symbol_short!("viol");

// ── Payment type ──────────────────────────────────────────────────────────────
pub const PAY_REWARD:  Symbol = symbol_short!("rew");
pub const PAY_PENALTY: Symbol = symbol_short!("pen");

// ── Rating tier ───────────────────────────────────────────────────────────────
pub const RATING_TOP:   Symbol = symbol_short!("top");
pub const RATING_EXCEL: Symbol = symbol_short!("excel");
pub const RATING_GOOD:  Symbol = symbol_short!("good");
pub const RATING_POOR:  Symbol = symbol_short!("poor");

// ── Severity levels ───────────────────────────────────────────────────────────
pub const SEV_CRITICAL: Symbol = symbol_short!("critical");
pub const SEV_HIGH:     Symbol = symbol_short!("high");
pub const SEV_MEDIUM:   Symbol = symbol_short!("medium");
pub const SEV_LOW:      Symbol = symbol_short!("low");
