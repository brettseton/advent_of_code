for dir in $(find . -name "Cargo.toml" -exec dirname {} \;); do
    echo "Processing $dir"
    cd "$dir"
    if ! cargo test --quiet --release; then
        echo "Tests failed in $dir"
        exit 1
    fi
    cd - > /dev/null
done