watch-test:
    watchexec -rcw src -- cargo test

watch-run:
    watchexec -rcw src -- cargo run --release
