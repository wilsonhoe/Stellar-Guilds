use soroban_sdk::{contracttype, Env, Vec};

use crate::milestone::types::{Milestone, Project};

#[contracttype]
pub enum MilestoneStorageKey {
    NextProjectId,
    NextMilestoneId,
    Project(u64),
    Milestone(u64),
    ProjectMilestones(u64), // Vec<milestone_id>
}

#[allow(dead_code)]
pub fn initialize_milestone_storage(env: &Env) {
    let storage = env.storage().persistent();
    if !storage.has(&MilestoneStorageKey::NextProjectId) {
        storage.set(&MilestoneStorageKey::NextProjectId, &1u64);
    }
    if !storage.has(&MilestoneStorageKey::NextMilestoneId) {
        storage.set(&MilestoneStorageKey::NextMilestoneId, &1u64);
    }
}

pub fn get_next_project_id(env: &Env) -> u64 {
    let mut next: u64 = env
        .storage()
        .persistent()
        .get(&MilestoneStorageKey::NextProjectId)
        .unwrap_or(1);
    let current = next;
    next += 1;
    env.storage()
        .persistent()
        .set(&MilestoneStorageKey::NextProjectId, &next);
    current
}

pub fn get_next_milestone_id(env: &Env) -> u64 {
    let mut next: u64 = env
        .storage()
        .persistent()
        .get(&MilestoneStorageKey::NextMilestoneId)
        .unwrap_or(1);
    let current = next;
    next += 1;
    env.storage()
        .persistent()
        .set(&MilestoneStorageKey::NextMilestoneId, &next);
    current
}

pub fn store_project(env: &Env, project: &Project) {
    env.storage()
        .persistent()
        .set(&MilestoneStorageKey::Project(project.id), project);
}

pub fn get_project(env: &Env, project_id: u64) -> Option<Project> {
    env.storage()
        .persistent()
        .get(&MilestoneStorageKey::Project(project_id))
}

pub fn store_milestone(env: &Env, milestone: &Milestone) {
    env.storage()
        .persistent()
        .set(&MilestoneStorageKey::Milestone(milestone.id), milestone);
}

pub fn get_milestone(env: &Env, milestone_id: u64) -> Option<Milestone> {
    env.storage()
        .persistent()
        .get(&MilestoneStorageKey::Milestone(milestone_id))
}

pub fn append_milestone_to_project(env: &Env, project_id: u64, milestone_id: u64) {
    let key = MilestoneStorageKey::ProjectMilestones(project_id);
    let mut ids: Vec<u64> = env
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env));
    ids.push_back(milestone_id);
    env.storage().persistent().set(&key, &ids);
}

pub fn get_project_milestone_ids(env: &Env, project_id: u64) -> Vec<u64> {
    let key = MilestoneStorageKey::ProjectMilestones(project_id);
    env.storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env))
}
