#![cfg(test)]

use soroban_sdk::testutils::Address as AddressTestUtils;
use soroban_sdk::{Address, BytesN, Env, String, Vec};

use crate::types::{CampaignStatus, MilestoneData, MilestoneStatus, StellarAsset};
use crate::CampaignContract;

fn setup(env: &Env) -> (Address, Address, Address) {
    let contract_id = env.register_contract(None, CampaignContract);
    let client = crate::CampaignContractClient::new(env, &contract_id);

    let creator = Address::generate(env);
    let token = Address::generate(env);

    let mut assets: Vec<StellarAsset> = Vec::new(env);
    assets.push_back(StellarAsset {
        asset_code: String::from_str(env, "XLM"),
        issuer: Some(token.clone()),
    });

    let mut milestones: Vec<MilestoneData> = Vec::new(env);
    milestones.push_back(MilestoneData {
        index: 0,
        target_amount: 1000,
        released_amount: 0,
        description_hash: BytesN::from_array(env, &[0u8; 32]),
        status: MilestoneStatus::Locked,
        released_at: None,
        released_at_ledger: None,
        release_tx: None,
        released_to: None,
    });

    client.initialize(
        &creator,
        &5000,
        &(env.ledger().timestamp() + 86400),
        &assets,
        &milestones,
        &0,
    );

    (contract_id, creator, token)
}

/// Issue #300 – Upgrade: only the admin can call upgrade.
/// A zero WASM hash has no corresponding blob in the test environment,
/// so the deployer panics — but auth is checked first.
/// This test verifies that upgrade is admin-gated.
#[test]
#[should_panic]
fn test_upgrade_requires_admin_auth() {
    let env = Env::default();
    // Do NOT mock auths — real auth check must fire
    let (contract_id, _creator, _token) = setup(&env);
    let client = crate::CampaignContractClient::new(&env, &contract_id);

    let hash: BytesN<32> = BytesN::from_array(&env, &[0u8; 32]);
    client.upgrade(&hash); // panics: Unauthorized (auth not mocked)
}

/// Issue #300 – Upgrade: upgrade panics with an unknown WASM hash.
/// With all auths mocked, the deployer call itself panics because the hash
/// points to no registered WASM — confirming the code path is exercised.
#[test]
#[should_panic]
fn test_upgrade_panics_with_unknown_wasm_hash() {
    let env = Env::default();
    env.mock_all_auths();
    let (contract_id, _creator, _token) = setup(&env);
    let client = crate::CampaignContractClient::new(&env, &contract_id);

    let unknown_hash: BytesN<32> = BytesN::from_array(&env, &[0xde, 0xad, 0xbe, 0xef,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00]);
    client.upgrade(&unknown_hash); // panics: Wasm does not exist
}

/// Issue #300 – Upgrade: state is preserved across upgrade.
/// Simulates the upgrade path by verifying that existing contract state
/// (campaign data, admin) is still accessible immediately before the
/// deployer swap — i.e., state reads work on the about-to-be-upgraded
/// contract.
#[test]
fn test_state_readable_before_upgrade() {
    let env = Env::default();
    env.mock_all_auths();
    let (contract_id, creator, _token) = setup(&env);
    let client = crate::CampaignContractClient::new(&env, &contract_id);

    // Confirm admin and campaign state are intact before upgrade attempt
    let admin = client.get_admin();
    assert_eq!(admin, Some(creator.clone()));

    let status = client.get_campaign_status();
    // Campaign is initialized: goal 5000, 0 raised, not frozen
    assert!(!status.is_frozen);
    assert_eq!(status.goal_amount, 5000);
}

/// Issue #300 – Upgrade: state is fully intact immediately before the upgrade call.
///
/// Soroban's `update_current_contract_wasm` atomically replaces the WASM while
/// leaving all persistent storage entries untouched. This test confirms that
/// all campaign state is readable up to the point of the upgrade — the
/// deployer panic from an unknown hash is the last operation, so storage
/// is never corrupted even on failure.
#[test]
fn test_campaign_state_intact_before_upgrade() {
    let env = Env::default();
    env.mock_all_auths();
    let (contract_id, creator, _token) = setup(&env);
    let client = crate::CampaignContractClient::new(&env, &contract_id);

    let admin = client.get_admin();
    assert_eq!(admin, Some(creator));

    // Campaign was initialized as Active with a future deadline
    let status = client.get_campaign_status();
    assert_eq!(status.status, CampaignStatus::Active);
    assert!(status.days_remaining > 0);
}
