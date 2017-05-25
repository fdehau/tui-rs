# Makefile for the tui-rs project (https://github.com/fdehau/tui-rs)


# ================================ Cargo ======================================


RUST_CHANNEL ?= stable
CARGO_FLAGS =
RUSTUP_INSTALLED = $(shell command -v rustup 2> /dev/null)

ifndef RUSTUP_INSTALLED
  CARGO = cargo
else
  ifdef NO_RUSTUP
    CARGO = cargo
  else
    CARGO = rustup run $(RUST_CHANNEL) cargo
  endif
endif


# ================================ Help =======================================


help: ## Print all the available commands
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
	  awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'


# ================================ Tools ======================================


install: install-rustfmt install-clippy ## Install tools dependencies

RUSTFMT_TARGET_VERSION = 0.8.4
RUSTFMT = $(shell command -v rustfmt 2> /dev/null)
ifeq ("$(RUSTFMT)","")
  RUSTFMT_INSTALL_CMD = @echo "Installing rustfmt $(RUSTFMT_TARGET_VERSION)" \
			&& $(CARGO) install --vers $(RUSTFMT_TARGET_VERSION) --force rustfmt
else
  RUSTFMT_CURRENT_VERSION = $(shell rustfmt --version | sed 's/^\(.*\) ()/\1/')
  ifeq ($(RUSTFMT_CURRENT_VERSION),$(RUSTFMT_TARGET_VERSION))
    RUSTFMT_INSTALL_CMD = @echo "Rustfmt is up to date"
  else
    RUSTFMT_INSTALL_CMD = @echo "Updating rustfmt from $(RUSTFMT_CURRENT_VERSION) to $(RUSTFMT_TARGET_VERSION)" \
			  && $(CARGO) install --vers $(RUSTFMT_TARGET_VERSION) --force rustfmt
  endif
endif

install-rustfmt: RUST_CHANNEL = nightly
install-rustfmt: ## Intall rustfmt
	$(RUSTFMT_INSTALL_CMD)


CLIPPY_TARGET_VERSION = 0.0.134
CLIPPY_CURRENT_VERSION = $(shell $(CARGO) clippy --version 2>/dev/null)
ifeq ("$(CLIPPY_CURRENT_VERSION)","")
  CLIPPY_INSTALL_CMD = @echo "Installing clippy $(CLIPPY_TARGET_VERSION)" \
		       && $(CARGO) install --vers $(CLIPPY_TARGET_VERSION) --force clippy
else
  ifeq ($(CLIPPY_CURRENT_VERSION),$(CLIPPY_TARGET_VERSION))
    CLIPPY_INSTALL_CMD = @echo "Clippy is up to date"
  else
    CLIPPY_INSTALL_CMD = @echo "Updating clippy from $(CLIPPY_CURRENT_VERSION) to $(CLIPPY_TARGET_VERSION)" \
			 && $(CARGO) install --vers $(CLIPPY_TARGET_VERSION) --force clippy
  endif
endif

install-clippy: RUST_CHANNEL = nightly
install-clippy: ## Install clippy
	$(CLIPPY_INSTALL_CMD)


# =============================== Build =======================================

check: ## Validate the project code
	$(CARGO) check

build: ## Build the project in debug mode
	$(CARGO) build $(CARGO_FLAGS)


release: CARGO_FLAGS += --release
release: build ## Build the project in release mode


# ================================ Lint =======================================

RUSTFMT_WRITEMODE ?= 'diff'

lint: fmt clippy ## Lint project files

fmt: ## Check the format of the source code
	$(CARGO) fmt -- --write-mode=$(RUSTFMT_WRITEMODE)

clippy: RUST_CHANNEL = nightly
clippy: ## Check the style of the source code and catch common errors
	$(CARGO) clippy --features="termion rustbox"


# ================================ Test =======================================


test: ## Run the tests
	$(CARGO) test

# ================================ Doc ========================================


doc: ## Build the documentation (available at ./target/doc)
	$(CARGO) doc


# ================================= Watch =====================================

# Requires watchman and watchman-make (https://facebook.github.io/watchman/docs/install.html)

watch: ## Watch file changes and build the project if any
	watchman-make -p 'src/**/*.rs' -t check build

watch-test: ## Watch files changes and run the tests if any
	watchman-make -p 'src/**/*.rs' 'tests/**/*.rs' 'examples/**/*.rs' -t test

watch-doc: ## Watch file changes and rebuild the documentation if any
	watchman-make -p 'src/**/*.rs' -t doc
