#!/bin/sh
cargo run < input.txt > actual.txt
diff expected.txt actual.txt