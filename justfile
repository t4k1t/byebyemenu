alias cov := coverage

# run unit tests
@test:
    echo "Running unit tests"
    cargo test

# generate coverage report
@coverage REPORT_TYPE='Html':
    cargo tarpaulin --out {{REPORT_TYPE}}

# clean dist
@clean:
    echo "Cleaning up existing artifacts…"
    cargo clean

# build dist
@build: clean
    echo "Building dist"
    cargo build --release

# publish to crates.io
@publish:
    echo "Publishing to crates.io"
    cargo login
    cargo publish

@debug:
    echo "Running with debug logging"
    RUST_LOG=debug cargo run

@preview-man:
    pandoc docs/src/byebyemenu.1.md -s -t man | man -l -

@build-man:
    pandoc docs/src/byebyemenu.1.md -s -t man | gzip --stdout - > docs/man/byebyemenu.1.gz
