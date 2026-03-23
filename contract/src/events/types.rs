/// Standardized event types for the Stellar Guilds platform.
///
/// Every event emitted across all contract modules is wrapped in a consistent
/// `EventEnvelope` that carries versioning, timing, and correlation metadata.
/// This ensures all events can be reliably indexed, filtered, and queried by
/// off-chain tooling, explorers, and monitoring systems.
///
/// # Event Schema Version
/// The current schema version is `1`. Bump this constant when any breaking
/// change is made to the envelope structure so consumers can handle both old
/// and new events.
///
/// # Topic Convention
/// All events use a two-element topic tuple: `(module, action)`.
/// Both are `Symbol` values with a fixed vocabulary defined in `topics.rs`.
///
/// # Usage
/// Never call `env.events().publish()` directly from module code.
/// Always go through `emit::emit_event()` so the envelope is populated
/// consistently and the sequence counter is incremented atomically.

use soroban_sdk::{contracttype, Symbol};

/// Current event schema version. Increment on any breaking envelope change.
pub const EVENT_SCHEMA_VERSION: u32 = 1;

/// Storage key used for the global monotonic event sequence counter.
/// Stored as a persistent instance value so it survives ledger closures.
pub const EVENT_SEQUENCE_KEY: &str = "evt_seq";

/// Metadata attached to every event published by this contract.
///
/// Fields:
/// - `version`   â€” Schema version; allows consumers to detect envelope changes.
/// - `timestamp` â€” Ledger timestamp at the moment of emission.
/// - `module`    â€” The sub-module that produced the event (e.g. `"bounty"`).
/// - `action`    â€” The specific action taken (e.g. `"created"`, `"funded"`).
/// - `sequence`  â€” Monotonically increasing counter scoped to this contract.
///                 Useful for ordering events when multiple are emitted in one
///                 transaction and for detecting gaps in off-chain listeners.
#[contracttype]
#[derive(Clone, Debug)]
pub struct EventEnvelope {
    /// Envelope schema version for backward-compatible consumer logic.
    pub version: u32,
    /// Ledger timestamp when the event was emitted (unix seconds).
    pub timestamp: u64,
    /// Module identifier, e.g. Symbol::new(env, "bounty").
    pub module: Symbol,
    /// Action identifier, e.g. Symbol::new(env, "created").
    pub action: Symbol,
    /// Contract-global monotonic sequence number.
    pub sequence: u64,
}
