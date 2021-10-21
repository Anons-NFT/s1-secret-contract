#!/bin/bash

make compile && sudo make compile-optimized-reproducible

secretclit tx compute store contract.wasm.gz --from test-account --gas 4000000 -y
