# Usage: sh blur.sh < white.png > synthwave.png

exec png2ff | cargo run --release --bin blur | ff2png
