# StellarAid Mainnet Launch Checklist

> Issue #302 — Every item must be signed off before mainnet deployment.

## Security

| # | Item | Owner | Status | Notes |
|---|------|-------|--------|-------|
| 1 | External smart contract audit complete | Security Lead | ⬜ Pending | Audit firm engaged |
| 2 | All critical and high severity findings resolved | Security Lead | ⬜ Pending | — |
| 3 | Formal verification proofs passing | Contract Team | ✅ Done | See `docs/formal-verification.md` |
| 4 | SQL injection / injection audit complete (API) | Backend Lead | ✅ Done | See `docs/sql-injection-audit.md` |
| 5 | OWASP Top 10 review complete | Security Lead | ⬜ Pending | — |

## Contract Readiness

| # | Item | Owner | Status | Notes |
|---|------|-------|--------|-------|
| 6 | Contract upgrade path tested on Testnet | Contract Team | ✅ Done | See `test/upgrade_tests.rs` |
| 7 | Emergency freeze / unfreeze tested on Testnet | Contract Team | ✅ Done | See `test/freeze_tests.rs` |
| 8 | Multi-sig admin configured (3-of-5) | DevOps Lead | ⬜ Pending | See `docs/admin-multisig.md` |
| 9 | All contract unit and integration tests passing | Contract Team | ⬜ Pending | `cargo test` must be green |
| 10 | Contract bytecode hash recorded for deployment manifest | Contract Team | ⬜ Pending | — |

## Testnet QA

| # | Item | Owner | Status | Notes |
|---|------|-------|--------|-------|
| 11 | Full campaign creation-to-payout flow tested | QA Lead | ⬜ Pending | — |
| 12 | Donation flow (XLM, USDC, AQUA) tested | QA Lead | ⬜ Pending | — |
| 13 | Milestone release tested end-to-end | QA Lead | ⬜ Pending | — |
| 14 | Refund flow tested within refund window | QA Lead | ⬜ Pending | — |
| 15 | Admin operations (freeze, upgrade, set_admin) verified | Contract Team | ⬜ Pending | — |

## Infrastructure

| # | Item | Owner | Status | Notes |
|---|------|-------|--------|-------|
| 16 | Production Dockerfile built and image size < 200 MB | DevOps Lead | ✅ Done | See `Dockerfile` in stellarAid-api |
| 17 | Docker Compose local dev stack functional | DevOps Lead | ✅ Done | `docker-compose up` verified |
| 18 | Monitoring and alerting configured (Sentry, Grafana) | DevOps Lead | ⬜ Pending | — |
| 19 | Database migration strategy approved for production | Backend Lead | ⬜ Pending | — |
| 20 | Environment variables documented and secrets stored in vault | DevOps Lead | ⬜ Pending | — |

## Incident Response

| # | Item | Owner | Status | Notes |
|---|------|-------|--------|-------|
| 21 | Incident response runbook written | Engineering Lead | ⬜ Pending | — |
| 22 | Emergency contact list established | Engineering Lead | ⬜ Pending | — |
| 23 | Contract freeze procedure documented and drilled | Contract Team | ⬜ Pending | — |
| 24 | On-call rotation set up | DevOps Lead | ⬜ Pending | — |

## Documentation

| # | Item | Owner | Status | Notes |
|---|------|-------|--------|-------|
| 25 | Contract interface reference published | Contract Team | ⬜ Pending | — |
| 26 | API documentation up to date (Swagger) | Backend Lead | ⬜ Pending | — |
| 27 | Admin multi-sig setup guide published | DevOps Lead | ✅ Done | See `docs/admin-multisig.md` |
| 28 | Upgrade guide published | Contract Team | ✅ Done | See `docs/upgrades.md` |

---

## Sign-off

| Role | Name | Date | Signature |
|------|------|------|-----------|
| Engineering Lead | | | |
| Security Lead | | | |
| Contract Team Lead | | | |
| DevOps Lead | | | |
| QA Lead | | | |

> **Launch gate**: All items must show ✅ Done and all sign-offs must be complete before mainnet deployment proceeds.
