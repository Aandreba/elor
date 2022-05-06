features := "serialize random macro async"

check:
	cargo check --no-default-features --all-targets
	cargo check --no-default-features --features alloc --all-targets
	cargo check --features $(features) --all-targets
	cargo check --all-targets

test:
	cargo test --no-default-features --all-targets
	cargo test --no-default-features --features alloc --all-targets
	cargo test --features $(features) --all-targets
	cargo test --all-targets

docs:
	rustup override set nightly
	cargo rustdoc --open --features $(features) -- --cfg docsrs
	rustup override set stable

publish: check test
	git add .
	git commit -m "Last commit before publish"
	git push
	cargo publish