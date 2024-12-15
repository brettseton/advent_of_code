for dir in $(find . -name "Cargo.toml" -exec dirname {} \;); do
    echo "Processing $dir"
    cd "$dir"
    echo "Running cargo fmt..."
    cargo fmt
    echo "Running cargo clippy..."
    cargo clippy -- -A clippy::needless_return -A clippy::redundant_field_names -A clippy::upper_case_acronyms -D warnings
    cd - > /dev/null
done