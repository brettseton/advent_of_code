for dir in $(find . -name "Cargo.toml" -exec dirname {} \;); do
    echo "Processing $dir"
    cd "$dir"
    echo "Running cargo fmt..."
    cargo fmt
    echo "Running cargo clippy..."
    cargo clippy -- -A clippy::needless_return -D warnings
    cd - > /dev/null
done