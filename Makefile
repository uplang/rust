# Makefile for UP Rust Parser

.PHONY: help
help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

.PHONY: test
test: ## Run tests
	cargo test --verbose

.PHONY: lint
lint: ## Run linter (clippy)
	cargo clippy -- -D warnings

.PHONY: build
build: test ## Build the library and binary
	cargo build --release

.PHONY: clean
clean: ## Clean build artifacts
	cargo clean

.PHONY: install
install: ## Install dependencies
	cargo fetch

.PHONY: fmt
fmt: ## Format code
	cargo fmt

.PHONY: check
check: ## Check code without building
	cargo check

.PHONY: doc
doc: ## Generate documentation
	cargo doc --no-deps --open

.PHONY: publish
publish: build test ## Publish to crates.io
	cargo publish

.PHONY: test-ci
test-ci: ## Run CI tests locally using act (requires: brew install act)
	act --container-architecture linux/amd64 -j test
	act --container-architecture linux/amd64 -j lint
	act --container-architecture linux/amd64 -j build

.DEFAULT_GOAL := build
