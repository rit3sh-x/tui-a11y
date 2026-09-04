default:
    @just --list

ci: fmt-check lint test doc deny machete minimal-versions

fmt:
    cargo +nightly fmt --all

fmt-check:
    cargo +nightly fmt --all -- --check

lint:
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo test --all-features
    cargo build --examples --all-features

doc:
    RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps

deny:
    cargo deny --all-features check

machete:
    cargo machete

minimal-versions:
    cargo minimal-versions check --direct --all-features --all-targets

list:
    cargo run --example list

dashboard:
    cargo run --example dashboard
