STATIC=$(pwd)/data/static
IMAGES=$(pwd)/data/images

cargo run --manifest-path backend/spar/Cargo.toml -- -p 8000 --folder "$STATIC" &
cargo run --manifest-path backend/images/Cargo.toml -- -p 8001 --folder "$IMAGES" &
catgo +nightly run --manifest-path backend/sessions/Cargo.toml -- -p 8002
