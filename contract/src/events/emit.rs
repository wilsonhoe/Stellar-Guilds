/// Central event emission helper for the Stellar Guilds contract.
///
/// All modules **must** emit events exclusively through `emit_event()`.
/// Direct calls to `env.events().publish()` are forbidden outside this file
/// to guarantee consistent envelope structure, sequence numbering, and topic
/// format across the entire contract.
///
/// # Example â€” emitting a guild creation event
/// ```rust
/// use crate::events::emit::emit_event;
/// use crate::events::topics::{MOD_GUILD, ACT_CREATED};
/// use crate::guild::types::GuildCreatedEvent;
///
/// emit_event(env, MOD_GUILD, ACT_CREATED, GuildCreatedEvent {
///     guild_id,
///     owner: owner.clone(),
///     name: name.clone(),
///     created_at: timestamp,
/// });
/// ```
///
/// # Sequence numbering
/// The global sequence is stored under the persistent `"evt_seq"` key and
/// incremented atomically on every emission. Off-chain indexers can use this
/// to detect missed events if their subscription lapsed.
///
/// # Size budget
/// Soroban charges per-byte for event data. Keep payload structs lean; use
/// IDs to reference large blobs stored elsewhere rather than inlining them.
use crate::events::types::{EventEnvelope, EVENT_SCHEMA_VERSION, EVENT_SEQUENCE_KEY};
use crate::integration::events::record_standardized_event;
use soroban_sdk::{Env, IntoVal, Symbol, Val};

/// Read the current global event sequence number.
/// Returns 0 if this is the first event ever emitted.
fn get_sequence(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get::<_, u64>(&Symbol::new(env, EVENT_SEQUENCE_KEY))
        .unwrap_or(0)
}

/// Persist the updated sequence number.
fn set_sequence(env: &Env, seq: u64) {
    env.storage()
        .instance()
        .set(&Symbol::new(env, EVENT_SEQUENCE_KEY), &seq);
}

/// Emit a standardized event with full envelope metadata.
///
/// # Type parameters
/// `T` â€” The event payload struct. Must implement `soroban_sdk::IntoVal<Env, Val>`,
/// which is automatically satisfied for any `#[contracttype]` struct.
///
/// # Arguments
/// * `env`    â€” The Soroban environment.
/// * `module` â€” String slice matching one of the `MOD_*` constants in `topics.rs`.
/// * `action` â€” String slice matching one of the `ACT_*` constants in `topics.rs`.
/// * `data`   â€” The event-specific payload struct.
///
/// # Topics
/// The published topic tuple is `(module_symbol, action_symbol)`, giving
/// Stellar's event filter system two independent axes to index on. Consumers
/// can subscribe to all events from a module, all events of an action type,
/// or the intersection of both.
/// emits event
pub fn emit_event<T>(env: &Env, module: &str, action: &str, data: T)
where
    T: IntoVal<Env, Val>,
{
    let seq = get_sequence(env) + 1;
    set_sequence(env, seq);

    let module_sym = Symbol::new(env, module);
    let action_sym = Symbol::new(env, action);

    let envelope = EventEnvelope {
        version: EVENT_SCHEMA_VERSION,
        timestamp: env.ledger().timestamp(),
        module: module_sym.clone(),
        action: action_sym.clone(),
        sequence: seq,
    };

    // Publish envelope on the meta-topic so indexers can track all events
    // without needing to know every module+action pair in advance.
    env.events().publish(
        (
            Symbol::new(env, "stellar_guilds"),
            Symbol::new(env, "event"),
        ),
        envelope,
    );

    // Publish the actual payload on the specific (module, action) topic so
    // consumers interested only in e.g. bounty:created can filter precisely.
    env.events().publish((module_sym, action_sym), data);

    record_standardized_event(env, module, action);
}

/// Convenience helper: emit an event and return a value in one expression.
///
/// Useful inside function bodies where you want to emit and then return:
/// ```rust
/// return emit_and_return(env, MOD_BOUNTY, ACT_CREATED, event, bounty_id);
/// ```
#[allow(dead_code)]
pub fn emit_and_return<T, R>(env: &Env, module: &str, action: &str, data: T, result: R) -> R
where
    T: IntoVal<Env, Val>,
{
    emit_event(env, module, action, data);
    result
}
