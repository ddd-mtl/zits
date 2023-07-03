#!/bin/bash

cargo run -- --no-hc-imports -d "import {ItemLink} from '../deps.types';" -i test/const/input.rs -o test/const/output.ts
cargo run -- --no-hc-imports -i test/struct/input.rs -o test/struct/output.ts
cargo run -- --default-zome-name useless -i test/type/input.rs -o test/type/output.ts
cargo run -- --no-hc-imports -i test/enum/input.rs -o test/enum/output.ts
cargo run -- --no-hc-imports -i test/fn/input.rs -o test/fn/output.ts
