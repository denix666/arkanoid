default: build_app

build_app:
	rm -rf target/release
	cargo build --release
	rm -rf ./arkanoid
	cp target/release/arkanoid .
