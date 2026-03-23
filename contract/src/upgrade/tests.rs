#![cfg(test)]

use super::types::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

fn create_test_version(major: u32, minor: u32, patch: u32) -> Version {
    Version::new(major, minor, patch)
}

#[test]
fn test_version_compatibility() {
    let _env = Env::default();
    
    let v1_0_0 = create_test_version(1, 0, 0);
    let v1_1_0 = create_test_version(1, 1, 0);
    let v2_0_0 = create_test_version(2, 0, 0);
    
    // Same version should be compatible
    assert!(v1_0_0.is_compatible_with(&v1_0_0));
    
    // Later minor version should be compatible
    assert!(v1_1_0.is_compatible_with(&v1_0_0));
    
    // Earlier minor version should not be compatible with later
    assert!(!v1_0_0.is_compatible_with(&v1_1_0));
    
    // Different major versions should not be compatible
    assert!(!v2_0_0.is_compatible_with(&v1_0_0));
    assert!(!v1_0_0.is_compatible_with(&v2_0_0));
}

#[test]
fn test_version_struct() {
    let version = create_test_version(1, 2, 3);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
}

#[test]
fn test_upgrade_status_enum() {
    let pending = UpgradeStatus::Pending;
    let approved = UpgradeStatus::Approved;
    let executed = UpgradeStatus::Executed;
    let rejected = UpgradeStatus::Rejected;
    let cancelled = UpgradeStatus::Cancelled;
    
    assert_eq!(pending as u32, 0);
    assert_eq!(approved as u32, 1);
    assert_eq!(executed as u32, 2);
    assert_eq!(rejected as u32, 3);
    assert_eq!(cancelled as u32, 4);
}

#[test]
fn test_migration_plan() {
    let env = Env::default();
    let from_version = create_test_version(1, 0, 0);
    let to_version = create_test_version(1, 1, 0);
    let selector = soroban_sdk::symbol_short!("migrate");
    
    let migration_plan = MigrationPlan {
        from_version,
        to_version,
        migration_function_selector: selector,
        estimated_gas: 100000,
    };
    
    assert_eq!(migration_plan.from_version.major, 1);
    assert_eq!(migration_plan.to_version.minor, 1);
    assert_eq!(migration_plan.estimated_gas, 100000);
}
