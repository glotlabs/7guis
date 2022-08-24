#!/bin/bash
set -e

cargo run -p sevenguis_cli -- home_page > dist/index.html
cargo run -p sevenguis_cli -- counter_page > dist/counter.html
cargo run -p sevenguis_cli -- temperature_page > dist/temperature.html
cargo run -p sevenguis_cli -- timer_page > dist/timer.html
cargo run -p sevenguis_cli -- crud_page > dist/crud.html
cargo run -p sevenguis_cli -- flight_page > dist/flight.html
