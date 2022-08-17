#!/bin/bash
set -e

echo "Cleaning up previous builds..."
mkdir -p dist
rm -rf dist/*

mkdir -p sevenguis_web/wasm
rm -rf sevenguis_web/wasm/*

echo
echo "Building workspace..."
cargo build

echo
echo "Building wasm..."
(
    cd sevenguis_wasm
    wasm-pack build --target web --out-name sevenguis --out-dir ../sevenguis_web/wasm
)

echo
echo "Generating html"
cargo run -p sevenguis_cli -- home_page > dist/index.html
cargo run -p sevenguis_cli -- temperature_page > dist/temperature.html
cargo run -p sevenguis_cli -- timer_page > dist/timer.html
cargo run -p sevenguis_cli -- crud_page > dist/crud.html
cargo run -p sevenguis_cli -- flight_page > dist/flight.html


echo
echo "Building web assets..."
(
    cd sevenguis_web
    npx tailwindcss --minify -i ./css/app.css -o ../dist/app.css
    npm run build-dev
    cp -rf wasm ../dist/
)

