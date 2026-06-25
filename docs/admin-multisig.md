# Admin Multi-Sig Configuration

> Issue #303 — Configures the on-chain admin address as a Stellar 3-of-5 multi-sig account so that no single key can unilaterally freeze, upgrade, or change the contract.

## Overview

The StellarAid contract admin address is a **3-of-5 multi-sig** Stellar account. Any admin operation (`freeze`, `unfreeze`, `upgrade`, `set_admin`) requires signatures from at least 3 of the 5 designated signers.

All signing keys are stored on **hardware wallets** held by different team members in different physical locations.

## Signer Configuration

| Signer # | Role | Device | Location |
|----------|------|--------|----------|
| 1 | Engineering Lead | Ledger Nano X | Office A |
| 2 | Security Lead | Trezor Model T | Office B |
| 3 | Contract Team Lead | Ledger Nano X | Remote |
| 4 | DevOps Lead | Ledger Nano S Plus | Office A |
| 5 | Executive Sponsor | Trezor Model T | Remote |

**Threshold**: 3 of 5 signers required.

## Setup Steps

### 1. Create the Multi-Sig Account

```bash
# Generate a new Stellar account (will become the multi-sig admin)
stellar keys generate admin-multisig --network mainnet

# Fund the account (minimum 1 XLM base reserve + signer reserves)
# Each additional signer costs 0.5 XLM base reserve
# With 5 signers: 1 + (5 × 0.5) = 3.5 XLM minimum
```

### 2. Add Signers

Using the [Stellar Laboratory](https://laboratory.stellar.org) or the Stellar CLI:

```bash
# Set master weight to 0 (disable the generating key)
# Add 5 signer public keys each with weight 1
# Set thresholds: low=1, med=3, high=3

stellar tx new set-options \
  --source-account GADMIN_MULTISIG_ADDRESS \
  --master-weight 0 \
  --low-threshold 1 \
  --med-threshold 3 \
  --high-threshold 3 \
  --signer "GSIGNER1_PUBKEY:1" \
  --signer "GSIGNER2_PUBKEY:1" \
  --signer "GSIGNER3_PUBKEY:1" \
  --signer "GSIGNER4_PUBKEY:1" \
  --signer "GSIGNER5_PUBKEY:1" \
  --network mainnet
```

### 3. Transfer Contract Admin

```bash
# Call set_admin on the contract to transfer admin rights
# Current admin (deployer) AND new multi-sig admin must both sign

stellar contract invoke \
  --id $CONTRACT_ID \
  --source deployer_key \
  --network mainnet \
  -- set_admin \
  --new_admin GADMIN_MULTISIG_ADDRESS
```

### 4. Verify the Configuration

```bash
# Confirm the admin is now the multi-sig account
stellar contract invoke \
  --id $CONTRACT_ID \
  --network mainnet \
  -- get_admin
```

Expected output: `GADMIN_MULTISIG_ADDRESS`

## Signing an Admin Transaction

When an admin operation is required (e.g., emergency freeze):

1. One signer constructs the transaction XDR offline.
2. They share the unsigned XDR with 2 other signers via secure channel.
3. Each signer signs independently with their hardware wallet.
4. Any signer assembles the 3-signature transaction envelope and submits.

```bash
# Example: emergency freeze requiring 3 signatures
# Step 1: Build transaction
stellar contract invoke ... -- freeze --build-only > freeze.xdr

# Step 2: Each signer adds their signature
stellar tx sign freeze.xdr --signer GSIGNER1_KEY >> freeze.xdr
stellar tx sign freeze.xdr --signer GSIGNER2_KEY >> freeze.xdr
stellar tx sign freeze.xdr --signer GSIGNER3_KEY >> freeze.xdr

# Step 3: Submit
stellar tx submit freeze.xdr --network mainnet
```

## Pre-Mainnet Test

Before mainnet deployment, a test multi-sig transaction on **Testnet** must be executed to verify the setup:

- [ ] 3-of-5 multi-sig account created on Testnet
- [ ] Thresholds set to low=1, med=3, high=3
- [ ] `set_admin` called successfully with Testnet multi-sig address
- [ ] Test `freeze` transaction constructed and signed by 3 signers
- [ ] `freeze` transaction submitted and confirmed on Testnet
- [ ] `unfreeze` transaction signed by 3 different signers and confirmed

## Security Notes

- Never store signing keys in software wallets or cloud key managers for multi-sig signers.
- The master key of the multi-sig account is disabled (weight 0) after setup.
- Loss of 3 or more hardware wallets simultaneously would require a key rotation using the remaining signers (if ≥ 3 remain available).
- Key rotation (`set_options` to replace a signer) also requires 3-of-5 approval.
