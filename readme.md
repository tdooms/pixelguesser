psql -d pixelguesser -a -f ./create.sql

- Create
    - exit + leave: quiz will be saved as draft
    - faulty form: give error on fields
    - continue: commit newQuiz
- Rounds
    - _: commit every few seconds with visual feedback


build --target web --out-name wasm --out-dir ../../data/static
run --manifest-path backend/spar/Cargo.toml -- -p 8000 --folder data/static
run --manifest-path backend/images/Cargo.toml -- -p 8001 --folder data/images
run --manifest-path backend/sessions/Cargo.toml -- -p 8002