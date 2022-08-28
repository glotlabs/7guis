#!/bin/bash
set -e


#!/bin/bash
set -e

env="$1"
event="$2"
target="debug"


if [[ "$env" == "release" ]]; then
    target="release"
fi


if [[ "$event" == "after_asset_hash" || "$env" == "dev" ]]; then
    # Generate html
    ./target/$target/sevenguis_cli home_page > dist/index.html
    ./target/$target/sevenguis_cli counter_page > dist/counter.html
    ./target/$target/sevenguis_cli temperature_page > dist/temperature.html
    ./target/$target/sevenguis_cli timer_page > dist/timer.html
    ./target/$target/sevenguis_cli crud_page > dist/crud.html
    ./target/$target/sevenguis_cli flight_page > dist/flight.html
fi

