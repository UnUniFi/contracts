#!/bin/bash

set -o errexit -o nounset

./ci-scripts/compile_and_optimize.sh

./ci-scripts/store_instantiate.sh

./ci-scripts/local-osmo/pool-creation.sh

./ci-scripts/msg.sh
