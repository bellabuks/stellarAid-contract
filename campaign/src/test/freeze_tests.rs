#![cfg(test)]

use soroban_sdk::testutils::Address as AddressTestUtils;
use soroban_sdk::{Address, BytesN, Env, String, Vec};

use crate::types::{AssetInfo, MilestoneData, MilestoneStatus, StellarAsset};
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

/// Issue #301 – Freeze: contract accepts donations before freeze.
#[test]
fn test_donate_succeeds_before_freeze() {
    let env = Env::default();
    env.mock_all_auths();
    let (contract_id, _creator, token) = setup(&env);
    let client = crate::CampaignContractClient::new(&env, &contract_id);

    let donor = Address::generate(&env);
    // Should not panic — contract is live
    client.donate(&donor, &500, &AssetInfo::Stellar(token));
}

/// Issue #301 – Freeze: admin can freeze the contract.
#[test]
fn test_admin_can_freeze() {
    let env = Env::default();
    env.mock_all_auths();
    let (contract_id, _creator, _token) = setup(&env);
    let client = crate::CampaignContractClient::new(&env, &contract_id);

    // freeze must not panic when called by admin
    client.freeze();
}

/// Issue #301 – Freeze: donations are rejected while contract is frozen.
#[test]
#[should_panic]
fn test_donate_panics_when_frozen() {
    let env = Env::default();
    env.mock_all_auths();
    let (contract_id, _creator, token) = setup(&env);
    let client = crate::CampaignContractClient::new(&env, &contract_id);

    let donor = Address::generate(&env);
    client.freeze();

    // Must panic: contract is frozen
    client.donate(&donor, &500, &AssetInfo::Stellar(token));
}

/// Issue #301 – Freeze: admin can unfreeze the contract.
#[test]
fn test_admin_can_unfreeze() {
    let env = Env::default();
    env.mock_all_auths();
    let (contract_id, _creator, _token) = setup(&env);
    let client = crate::CampaignContractClient::new(&env, &contract_id);

    client.freeze();
    // Must not panic
    client.unfreeze();
}

/// Issue #301 – Freeze: donations succeed again after unfreeze.
#[test]
fn test_donate_succeeds_after_unfreeze() {
    let env = Env::default();
    env.mock_all_auths();
    let (contract_id, _creator, token) = setup(&env);
    let client = crate::CampaignContractClient::new(&env, &contract_id);

    let donor = Address::generate(&env);

    client.freeze();
    client.unfreeze();

    // Must not panic — contract is live again
    client.donate(&donor, &500, &AssetInfo::Stellar(token));
}

/// Issue #301 – Freeze: freeze is idempotent (freezing twice does not panic).
#[test]
fn test_freeze_is_idempotent() {
    let env = Env::default();
    env.mock_all_auths();
    let (contract_id, _creator, _token) = setup(&env);
    let client = crate::CampaignContractClient::new(&env, &contract_id);

    client.freeze();
    client.freeze(); // second freeze must not panic
}

/// Issue #301 – Freeze: only the admin can freeze.
#[test]
#[should_panic]
fn test_non_admin_cannot_freeze() {
    let env = Env::default();
    // Do NOT mock auths — let real auth check fire
    let (contract_id, _creator, _token) = setup(&env);
    let client = crate::CampaignContractClient::new(&env, &contract_id);

    // No auth mocked → admin.require_auth() will panic
    client.freeze();
}
