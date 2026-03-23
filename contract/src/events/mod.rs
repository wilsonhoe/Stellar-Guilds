/// Centralized event logging system for Stellar Guilds.
///
/// # Problem solved
/// Prior to this module, each sub-module emitted events independently with
/// inconsistent topic formats:
/// - `guild/membership.rs`: `(Symbol::new(env, "guild_created"), Symbol::new(env, "v0"))`
/// - `bounty/mod.rs`:       `(Symbol::new(env, "bounty"), Symbol::new(env, "created"))`
/// - `payment/distribution.rs`: `("PaymentPoolCreated",)` â€” a plain string
///
/// This made off-chain indexing fragile, broke any tooling trying to subscribe
/// to a consistent event stream, and made audit trails incomplete.
///
/// # Architecture
/// ```
/// events/
///   mod.rs    â† you are here; re-exports for easy import
///   types.rs  â† EventEnvelope + schema version constant
///   topics.rs â† MOD_* and ACT_* string constants (the controlled vocabulary)
///   emit.rs   â† emit_event() â€” the only function modules should call
/// ```
///
/// # Two-event-per-action design
/// `emit_event()` publishes **two** Soroban events per user action:
///
/// 1. A meta-event on topic `(stellar_guilds, event)` carrying the
///    `EventEnvelope` (version, timestamp, module, action, sequence).
///    Off-chain indexers subscribe here to receive *all* events from the
///    contract in a single stream without knowing every topic pair.
///
/// 2. A payload event on topic `(module, action)` carrying the module-specific
///    data struct. Consumers that care only about e.g. bounty:funded filter
///    directly on this topic for maximum efficiency.
///
/// # Backward compatibility
/// - `EVENT_SCHEMA_VERSION` (currently `1`) is embedded in every envelope.
///   When the envelope shape changes, bump this constant and update consumers.
/// - Topic symbol strings in `topics.rs` must not be renamed once deployed;
///   treat them as stable API surface.
/// - Per-module event payload structs live in each module's own `types.rs`
///   and may evolve independently; version them via the module-level
///   schema if needed.
///
/// # Gas considerations
/// Each `env.events().publish()` call has a per-byte cost. The envelope is
/// small (5 fields, ~80 bytes). Keep payload structs lean; use IDs to
/// reference large stored data rather than inlining it.
///
/// # Adding a new module
/// 1. Add `MOD_<MODULE>` to `topics.rs`.
/// 2. Add `ACT_*` constants for every action in `topics.rs`.
/// 3. Define payload structs (with `#[contracttype]`) in the module's
///    own `types.rs`.
/// 4. Replace every `env.events().publish(...)` call in the module with
///    `emit_event(env, MOD_<MODULE>, ACT_<ACTION>, payload)`.
/// 5. Document the new events in the module's top-level doc comment.

pub mod emit;
pub mod topics;
pub mod types;

// Convenience re-exports so module code only needs one import line:
// `use crate::events::{emit_event, topics::*};`
pub use emit::emit_event;
