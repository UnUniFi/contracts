#!/bin/bash

set -o errexit -o nounset

export SCRIPT_DIR=$(cd $(dirname $0); pwd)
source $SCRIPT_DIR/env

$SCRIPT_DIR/compile_and_optimize.sh

$SCRIPT_DIR/store_instantiate.sh

$SCRIPT_DIR/pool-creation.sh

$SCRIPT_DIR/msg.sh
