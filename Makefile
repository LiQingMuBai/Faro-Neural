SHELL := /bin/sh

.DEFAULT_GOAL := help

NPM ?= npm
CARGO ?= cargo
BOT_MANIFEST := bot/Cargo.toml

.PHONY: help install dev build preview bot bot-check bot-build check release clean

help: ## Show available commands
	@printf '%s\n' 'Faro Neural build commands:'
	@awk 'BEGIN {FS = ":.*## "} /^[a-zA-Z0-9_-]+:.*## / {printf "  %-12s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

install: ## Install website dependencies and fetch Rust dependencies
	$(NPM) install
	$(CARGO) fetch --manifest-path $(BOT_MANIFEST)

dev: ## Start the website development server
	$(NPM) run dev

build: ## Build the website and release-mode Rust bot
	$(NPM) run build
	$(CARGO) build --release --locked --manifest-path $(BOT_MANIFEST)

preview: ## Preview the production website build
	$(NPM) run preview

bot: ## Run the Rust Telegram bot in release mode
	$(CARGO) run --release --locked --manifest-path $(BOT_MANIFEST)

bot-check: ## Format-check and type-check the Rust bot
	$(CARGO) fmt --manifest-path $(BOT_MANIFEST) -- --check
	$(CARGO) check --locked --manifest-path $(BOT_MANIFEST)

bot-build: ## Build only the release-mode Rust bot
	$(CARGO) build --release --locked --manifest-path $(BOT_MANIFEST)

check: bot-check ## Validate both website and bot
	$(NPM) run build

release: clean build ## Produce clean production artifacts

clean: ## Remove generated website and Rust build artifacts
	$(NPM) exec --yes rimraf dist 2>/dev/null || rm -rf dist
	$(CARGO) clean --manifest-path $(BOT_MANIFEST)
