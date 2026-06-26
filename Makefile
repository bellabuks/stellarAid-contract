build:
	cargo build --target wasm32-unknown-unknown --release

test:
	cargo test

deploy-testnet:
	./scripts/deploy.sh testnet

deploy-mainnet:
	./scripts/deploy.sh mainnet

.PHONY: build test deploy-testnet deploy-mainnet
