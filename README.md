# StellarAid Contract

Rust workspace for StellarAid Soroban smart contracts.

## Structure

```
contracts/
  donation/     # Donation smart contract
  withdrawal/   # Withdrawal smart contract
  campaign/     # Campaign smart contract
sdk/            # Shared SDK (Horizon client, RPC, utilities)
worker/         # Background worker
scripts/        # Deployment scripts
docs/           # Documentation
```

## Quick Start

```bash
cp .env.example .env
make build
make test
```

See [docs/DEPLOY.md](docs/DEPLOY.md) for deployment instructions.
