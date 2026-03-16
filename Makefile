FEATURES := \
	min-usize-32 \
	min-usize-64 \
	from_usize \
	min-usize-32,from_usize \
	min-usize-64,from_usize

DOC_FEATURES_32 := min-usize-32,from_usize
DOC_FEATURES_64 := min-usize-64,from_usize

TARGET_POINTER_WIDTH := $(shell rustc --print cfg | sed -n 's/^target_pointer_width="\([0-9]*\)"/\1/p')
MSRV := $(shell sed -n 's/^rust-version *= *"\(.*\)"/\1/p' Cargo.toml)

ifeq ($(TARGET_POINTER_WIDTH),32)
DOC_FEATURES := $(DOC_FEATURES_32)
else
DOC_FEATURES := $(DOC_FEATURES_64)
endif

.PHONY: all clean test doctest clippy fmt ci doc check-msrv-badge

all: ci

clean:
	cargo clean

test:
	@for f in $(FEATURES); do \
		case "$(TARGET_POINTER_WIDTH):$$f" in \
			32:min-usize-64|32:min-usize-64,from_usize) \
				echo "expect fail: cargo check --features $$f"; \
				if cargo check --features "$$f" >/dev/null 2>&1; then \
					echo "Expected failure for $$f on $(TARGET_POINTER_WIDTH)-bit target"; \
					exit 1; \
				else \
					echo "Got expected failure for $$f on $(TARGET_POINTER_WIDTH)-bit target"; \
				fi; \
				;; \
			*) \
				echo "expect pass: cargo test --lib --features $$f"; \
				cargo test --lib --features "$$f"; \
				;; \
		esac; \
	done

doctest:
	cargo test --doc --features "$(DOC_FEATURES)"

clippy:
	@for f in $(FEATURES); do \
		case "$(TARGET_POINTER_WIDTH):$$f" in \
			32:min-usize-64|32:min-usize-64,from_usize) \
				echo "expect fail: cargo clippy --features $$f"; \
				if cargo clippy --features "$$f" -- -D warnings >/dev/null 2>&1; then \
					echo "Expected clippy failure for $$f on $(TARGET_POINTER_WIDTH)-bit target"; \
					exit 1; \
				else \
					echo "Got expected clippy failure for $$f on $(TARGET_POINTER_WIDTH)-bit target"; \
				fi; \
				;; \
			*) \
				echo "expect pass: cargo clippy --features $$f"; \
				cargo clippy --features "$$f" -- -D warnings; \
				;; \
		esac; \
	done

fmt:
	cargo fmt --check

ci: fmt clippy check-msrv-badge test doctest

doc:
	cargo doc --features "$(DOC_FEATURES)" --no-deps --open

check-msrv-badge:
	@grep -q "MSRV-$(MSRV)-blue" README.md || \
		( echo "MSRV badge mismatch (Cargo.toml: $(MSRV))"; exit 1 )
