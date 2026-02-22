.DEFAULT_GOAL := help

##@ Development

.PHONY: build
build: ## Build debug binary
	cargo build

.PHONY: release
release: ## Build release binary
	cargo build --release

.PHONY: test
test: ## Run tests
	cargo test

##@ Code Quality

.PHONY: check
check: fmt-check clippy ## Run fmt check and clippy

.PHONY: fix
fix: fmt-fix clippy-fix ## Auto-fix fmt and clippy issues

.PHONY: fmt
fmt: ## Format code
	cargo fmt

.PHONY: fmt-check
fmt-check: ## Check formatting
	cargo fmt --check

.PHONY: fmt-fix
fmt-fix: ## Fix formatting
	cargo fmt

.PHONY: clippy
clippy: ## Run clippy lints
	cargo clippy -- -D warnings

.PHONY: clippy-fix
clippy-fix: ## Auto-fix clippy issues
	cargo clippy --fix --allow-dirty -- -D warnings

##@ Maintenance

.PHONY: clean
clean: ## Remove build artifacts
	cargo clean

##@ Info

.PHONY: help
help: ## Show this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} \
		/^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } \
		/^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)
