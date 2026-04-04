# Neo Rust Node Makefile
# R3E Network <jimmy@r3e.network>

# Variables
CARGO = cargo
DOCKER = docker
DOCKER_IMAGE = neo-rust-node
DOCKER_TAG = latest
RELEASE_DIR = target/release
DEBUG_DIR = target/debug
DATA_DIR = data

# Color codes for output
GREEN = \033[0;32m
YELLOW = \033[0;33m
RED = \033[0;31m
NC = \033[0m # No Color

# Default target
.PHONY: help
help:
	@echo "$(GREEN)Neo Rust Node - Available Commands:$(NC)"
	@echo ""
	@echo "$(YELLOW)Building:$(NC)"
	@echo "  make build          - Build the node in debug mode"
	@echo "  make build-release  - Build the node in release mode"
	@echo "  make clean          - Clean build artifacts"
	@echo ""
	@echo "$(YELLOW)Running:$(NC)"
	@echo "  make run            - Run neo-node on mainnet"
	@echo "  make run testnet    - Run neo-node on testnet"
	@echo "  make run docker     - Run neo-node in Docker container"
	@echo "  make run-release    - Run neo-node in release mode"
	@echo "  make run-daemon     - Run neo-node in daemon mode"
	@echo ""
	@echo "$(YELLOW)Docker:$(NC)"
	@echo "  make docker         - Build Docker image"
	@echo "  make docker-run     - Run Docker container"
	@echo "  make docker-stop    - Stop Docker container"
	@echo "  make docker-logs    - View Docker logs"
	@echo "  make docker-clean   - Remove Docker image"
	@echo ""
	@echo "$(YELLOW)Development:$(NC)"
	@echo "  make test           - Run all tests"
	@echo "  make test-unit      - Run unit tests"
	@echo "  make test-integration - Run integration tests"
	@echo "  make fmt            - Format code"
	@echo "  make clippy         - Run clippy linter"
	@echo "  make check          - Check code without building"
	@echo "  make doc            - Generate documentation"
	@echo ""
	@echo "$(YELLOW)Database:$(NC)"
	@echo "  make db-clean       - Clean blockchain database"
	@echo "  make db-backup      - Backup blockchain database"
	@echo "  make db-restore     - Restore blockchain database"
	@echo ""
	@echo "$(YELLOW)Release:$(NC)"
	@echo "  make release        - Create release binaries"
	@echo "  make dist           - Create distribution package"

# Building targets
.PHONY: build
build:
	@echo "$(GREEN)Building Neo node in debug mode...$(NC)"
	$(CARGO) build --workspace
	@echo "$(GREEN)Build complete!$(NC)"

.PHONY: build-release
build-release:
	@echo "$(GREEN)Building Neo node in release mode...$(NC)"
	$(CARGO) build --release --workspace
	@echo "$(GREEN)Release build complete!$(NC)"

.PHONY: clean
clean:
	@echo "$(YELLOW)Cleaning build artifacts...$(NC)"
	$(CARGO) clean
	@echo "$(GREEN)Clean complete!$(NC)"

# Running targets
.PHONY: run
run: build-release
	@echo "$(GREEN)Starting Neo Node on MainNet...$(NC)"
	./$(RELEASE_DIR)/neo-node --network mainnet

.PHONY: run-release
run-release: build-release
	@echo "$(GREEN)Starting Neo Node (release)...$(NC)"
	./$(RELEASE_DIR)/neo-node

.PHONY: run-testnet
run-testnet: build
	@echo "$(GREEN)Starting Neo Node on TestNet...$(NC)"
	./$(DEBUG_DIR)/neo-node --network testnet

.PHONY: run-mainnet
run-mainnet: build-release
	@echo "$(GREEN)Starting Neo Node on MainNet...$(NC)"
	./$(RELEASE_DIR)/neo-node --network mainnet

.PHONY: run-daemon
run-daemon: build
	@echo "$(GREEN)Starting Neo Node in daemon mode...$(NC)"
	./$(DEBUG_DIR)/neo-node --network testnet --daemon

# Docker targets
.PHONY: docker
docker:
	@echo "$(GREEN)Building Docker image...$(NC)"
	$(DOCKER) build -t $(DOCKER_IMAGE):$(DOCKER_TAG) .
	@echo "$(GREEN)Docker image built: $(DOCKER_IMAGE):$(DOCKER_TAG)$(NC)"

.PHONY: docker-run
docker-run:
	@echo "$(GREEN)Running Docker container...$(NC)"
	$(DOCKER) run -d \
		--name neo-rust-node \
		-p 20333:20333 \
		-p 20334:20334 \
		-p 20332:20332 \
		-v $(PWD)/$(DATA_DIR):/data \
		$(DOCKER_IMAGE):$(DOCKER_TAG)
	@echo "$(GREEN)Container started: neo-rust-node$(NC)"

.PHONY: docker-stop
docker-stop:
	@echo "$(YELLOW)Stopping Docker container...$(NC)"
	$(DOCKER) stop neo-rust-node || true
	$(DOCKER) rm neo-rust-node || true
	@echo "$(GREEN)Container stopped$(NC)"

.PHONY: docker-logs
docker-logs:
	@echo "$(GREEN)Viewing Docker logs...$(NC)"
	$(DOCKER) logs -f neo-rust-node

.PHONY: docker-clean
docker-clean: docker-stop
	@echo "$(YELLOW)Removing Docker image...$(NC)"
	$(DOCKER) rmi $(DOCKER_IMAGE):$(DOCKER_TAG) || true
	@echo "$(GREEN)Docker image removed$(NC)"

# Development targets
.PHONY: test
test:
	@echo "$(GREEN)Running all tests...$(NC)"
	$(CARGO) test --workspace

.PHONY: test-unit
test-unit:
	@echo "$(GREEN)Running unit tests...$(NC)"
	$(CARGO) test --workspace --lib

.PHONY: test-integration
test-integration:
	@echo "$(GREEN)Running integration tests...$(NC)"
	$(CARGO) test --workspace --test '*'

.PHONY: fmt
fmt:
	@echo "$(GREEN)Formatting code...$(NC)"
	$(CARGO) fmt --all

.PHONY: clippy
clippy:
	@echo "$(GREEN)Running clippy...$(NC)"
	$(CARGO) clippy --workspace --all-targets --all-features -- -D warnings

.PHONY: check
check:
	@echo "$(GREEN)Checking code...$(NC)"
	$(CARGO) check --workspace --all-targets

.PHONY: doc
doc:
	@echo "$(GREEN)Generating documentation...$(NC)"
	$(CARGO) doc --workspace --no-deps --open

# Database targets
.PHONY: db-clean
db-clean:
	@echo "$(YELLOW)Cleaning blockchain database...$(NC)"
	@read -p "Are you sure you want to delete the blockchain data? [y/N] " confirm; \
	if [ "$$confirm" = "y" ]; then \
		rm -rf $(DATA_DIR)/blocks $(DATA_DIR)/state; \
		echo "$(GREEN)Database cleaned$(NC)"; \
	else \
		echo "$(YELLOW)Cancelled$(NC)"; \
	fi

.PHONY: db-backup
db-backup:
	@echo "$(GREEN)Backing up blockchain database...$(NC)"
	@mkdir -p backups
	tar -czf backups/neo-db-backup-$(shell date +%Y%m%d-%H%M%S).tar.gz $(DATA_DIR)/
	@echo "$(GREEN)Backup complete$(NC)"

.PHONY: db-restore
db-restore:
	@echo "$(GREEN)Restoring blockchain database...$(NC)"
	@echo "Available backups:"
	@ls -la backups/neo-db-backup-*.tar.gz 2>/dev/null || echo "No backups found"
	@read -p "Enter backup filename to restore: " backup; \
	if [ -f "$$backup" ]; then \
		tar -xzf $$backup; \
		echo "$(GREEN)Restore complete$(NC)"; \
	else \
		echo "$(RED)Backup file not found$(NC)"; \
	fi

# Release targets
.PHONY: release
release: build-release
	@echo "$(GREEN)Creating release binaries...$(NC)"
	@mkdir -p dist/bin
	cp $(RELEASE_DIR)/neo-node dist/bin/
	@echo "$(GREEN)Release binaries created in dist/bin/$(NC)"

.PHONY: dist
dist: release
	@echo "$(GREEN)Creating distribution package...$(NC)"
	@mkdir -p dist/config dist/scripts
	@cp -r config/* dist/config/ 2>/dev/null || true
	@cp -r scripts/* dist/scripts/ 2>/dev/null || true
	tar -czf neo-rust-node-$(shell date +%Y%m%d).tar.gz -C dist .
	@echo "$(GREEN)Distribution package created: neo-rust-node-$(shell date +%Y%m%d).tar.gz$(NC)"

# Phony targets
.PHONY: all
all: build test

.PHONY: ci
ci: check fmt clippy test

# Convenience aliases for multi-word targets
.PHONY: testnet
testnet: run-testnet

