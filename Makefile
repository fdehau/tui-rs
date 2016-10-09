build:
	cargo build
test:
	cargo test
watch:
	watchman-make -p 'src/**/*.rs' -t build -p 'test/**/*.rs' -t test
