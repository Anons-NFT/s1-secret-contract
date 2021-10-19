#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck >/dev/null && shellcheck "$0"

make compile && sudo make compile-optimized-reproducible

secretcli tx compute store contract.wasm.gz --from secret1lwt9xk3q9s556tuw6p9s5s7y706l6r6l7gxfpc --gas 4000000 -y

secretcli query compute list-code | tail -20
