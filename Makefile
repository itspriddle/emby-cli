.PHONY: build release test check fix fmt fmt-check fmt-fix clippy clippy-fix clean help

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-14s\033[0m %s\n", $$1, $$2}'

build: ## Build debug binary
	cargo build

release: ## Build release binary
	cargo build --release

test: ## Run tests
	cargo test

check: fmt-check clippy ## Run fmt check and clippy

fix: fmt-fix clippy-fix ## Auto-fix fmt and clippy issues

fmt: ## Format code
	cargo fmt

fmt-check: ## Check formatting
	cargo fmt --check

fmt-fix: ## Fix formatting
	cargo fmt

clippy: ## Run clippy lints
	cargo clippy -- -D warnings

clippy-fix: ## Auto-fix clippy issues
	cargo clippy --fix --allow-dirty -- -D warnings

clean: ## Remove build artifacts
	cargo clean
