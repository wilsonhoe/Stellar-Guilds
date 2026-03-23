use crate::guild::types::{Guild, Member, Role};
use soroban_sdk::{symbol_short, Address, Env, Map, Symbol, Vec};

// Storage keys as symbols for efficient lookup
const GUILDS_KEY: Symbol = symbol_short!("guilds");
const MEMBERS_KEY: Symbol = symbol_short!("members");
const GUILD_COUNTER_KEY: Symbol = symbol_short!("guild_cnt");

/// Initialize storage for guilds and members
/// This should be called during contract initialization
pub fn initialize(env: &Env) {
    // Create an empty map for guilds
    let guilds: Map<u64, Guild> = Map::new(env);
    env.storage().persistent().set(&GUILDS_KEY, &guilds);

    // Create an empty nested map for members
    // Structure: guild_id -> (address -> member)
    let members: Map<u64, Map<Address, Member>> = Map::new(env);
    env.storage().persistent().set(&MEMBERS_KEY, &members);

    // Initialize guild counter to 0
    env.storage().persistent().set(&GUILD_COUNTER_KEY, &0u64);
}

/// Get the next guild ID and increment the counter
pub fn get_next_guild_id(env: &Env) -> u64 {
    let counter: u64 = env
        .storage()
        .persistent()
        .get(&GUILD_COUNTER_KEY)
        .unwrap_or(0u64);

    let next_id = counter + 1;
    env.storage().persistent().set(&GUILD_COUNTER_KEY, &next_id);

    next_id
}

/// Store a guild
pub fn store_guild(env: &Env, guild: &Guild) {
    let mut guilds: Map<u64, Guild> = env
        .storage()
        .persistent()
        .get(&GUILDS_KEY)
        .unwrap_or_else(|| Map::new(env));

    guilds.set(guild.id, guild.clone());
    env.storage().persistent().set(&GUILDS_KEY, &guilds);
}

/// Get a guild by ID
pub fn get_guild(env: &Env, guild_id: u64) -> Option<Guild> {
    let guilds: Map<u64, Guild> = env
        .storage()
        .persistent()
        .get(&GUILDS_KEY)
        .unwrap_or_else(|| Map::new(env));

    guilds.get(guild_id)
}

/// Store a member in a guild
pub fn store_member(env: &Env, guild_id: u64, member: &Member) {
    let mut members_map: Map<u64, Map<Address, Member>> = env
        .storage()
        .persistent()
        .get(&MEMBERS_KEY)
        .unwrap_or_else(|| Map::new(env));

    let mut guild_members = members_map.get(guild_id).unwrap_or_else(|| Map::new(env));

    guild_members.set(member.address.clone(), member.clone());
    members_map.set(guild_id, guild_members);

    env.storage().persistent().set(&MEMBERS_KEY, &members_map);
}

/// Get a member from a guild
pub fn get_member(env: &Env, guild_id: u64, address: &Address) -> Option<Member> {
    let members_map: Map<u64, Map<Address, Member>> = env
        .storage()
        .persistent()
        .get(&MEMBERS_KEY)
        .unwrap_or_else(|| Map::new(env));

    let guild_members = members_map.get(guild_id)?;
    guild_members.get(address.clone())
}

/// Remove a member from a guild
pub fn remove_member(env: &Env, guild_id: u64, address: &Address) -> bool {
    let mut members_map: Map<u64, Map<Address, Member>> = env
        .storage()
        .persistent()
        .get(&MEMBERS_KEY)
        .unwrap_or_else(|| Map::new(env));

    if let Some(mut guild_members) = members_map.get(guild_id) {
        // Remove the member
        let had_member = guild_members.contains_key(address.clone());
        if had_member {
            guild_members.remove(address.clone());
            members_map.set(guild_id, guild_members);
            env.storage().persistent().set(&MEMBERS_KEY, &members_map);
        }
        had_member
    } else {
        false
    }
}

/// Get all members of a guild
pub fn get_all_members(env: &Env, guild_id: u64) -> Vec<Member> {
    let members_map: Map<u64, Map<Address, Member>> = env
        .storage()
        .persistent()
        .get(&MEMBERS_KEY)
        .unwrap_or_else(|| Map::new(env));

    if let Some(guild_members) = members_map.get(guild_id) {
        let mut result = Vec::new(env);

        // Iterate through all members in the guild using iter()
        for (_, member) in guild_members.iter() {
            result.push_back(member);
        }

        result
    } else {
        Vec::new(env)
    }
}

/// Check if a member exists in a guild
pub fn has_member(env: &Env, guild_id: u64, address: &Address) -> bool {
    get_member(env, guild_id, address).is_some()
}

/// Update a guild's metadata
pub fn update_guild(env: &Env, guild: &Guild) {
    let mut guilds: Map<u64, Guild> = env
        .storage()
        .persistent()
        .get(&GUILDS_KEY)
        .unwrap_or_else(|| Map::new(env));

    guilds.set(guild.id, guild.clone());
    env.storage().persistent().set(&GUILDS_KEY, &guilds);
}

/// Count owners in a guild
pub fn count_owners(env: &Env, guild_id: u64) -> u32 {
    let members = get_all_members(env, guild_id);
    let mut count = 0u32;

    for i in 0..members.len() {
        let member = members.get_unchecked(i);
        if member.role == Role::Owner {
            count += 1;
        }
    }

    count
}
