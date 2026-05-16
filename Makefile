SHELL := /bin/bash

.DEFAULT_GOAL := help

.PHONY: help fmt fmt-check check test test-core test-http test-storage test-import check-ui run-ui clean tree

help: ## Show available commands
	@awk 'BEGIN {FS = ":.*##"; printf "Birnio developer commands\n\n"} /^[a-zA-Z0-9_-]+:.*?##/ {printf "  \033[36m%-16s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

fmt: ## Format the workspace
	cargo fmt --all

fmt-check: ## Check formatting without writing files
	cargo fmt --all --check

check: ## Type-check the full workspace
	cargo check --workspace

test: ## Run all tests
	cargo test --workspace

test-core: ## Run birnio-core tests
	cargo test -p birnio-core

test-http: ## Run birnio-http tests
	cargo test -p birnio-http

test-storage: ## Run birnio-storage tests
	cargo test -p birnio-storage

test-import: ## Run birnio-import tests
	cargo test -p birnio-import

check-ui: ## Type-check the GTK/libadwaita UI
	cargo check -p birnio-ui-gtk

run-ui: ## Run the GTK/libadwaita UI
	cargo run -p birnio-ui-gtk

tree: ## Print the workspace dependency tree
	cargo tree --workspace --depth 2

clean: ## Remove build artifacts
	cargo clean
