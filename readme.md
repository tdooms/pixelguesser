
# Clion config
**serve index:** run --manifest-path backend/spar/Cargo.toml -- -p 8900 --folder data/static

**serve images:** run --manifest-path backend/imager/Cargo.toml -- -p 8901 --folder data/images

**start session manager:** run --features "full" --manifest-path backend/sessions/Cargo.toml -- -p 8902

**generate frontend:** build --target web --dev --out-name wasm --out-dir ../data/static

# Temp
- Overview 
  - Simple overview searchable by name and description. (priority low)
  - 

