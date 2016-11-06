build:
	cargo build
test:
	cargo test
doc:
	cargo doc
watch:
	watchman-make -p 'src/**/*.rs' -t build -p 'test/**/*.rs' -t test

watch-test:
	watchman-make -p 'src/**/*.rs' 'tests/**/*.rs' 'examples/**/*.rs' -t test

watch-doc:
	watchman-make -p 'src/**/*.rs' -t doc
