default:
    just --list

format:
    cargo fmt --all

check-format:
    cargo fmt --all -- --check

doc $RUSTDOCFLAGS="-D warnings":
    cargo doc --no-deps --document-private-items

clippy:
    cargo clippy --all-targets --all-features -- -D warnings -W clippy::pedantic

test:
    cargo test

ci: check-format doc clippy test
