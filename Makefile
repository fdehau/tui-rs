build:
	cargo build
test:
	cargo test
watch:
	watchman-make -p 'src/**/*.rs' -t build -p 'test/**/*.rs' -t test

watch-test:
	watchman-make -p 'src/**/*.rs' 'tests/**/*.rs' -t test
