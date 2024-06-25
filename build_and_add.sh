#!/bin/bash
# shellcheck disable=SC2034

echo "[->] Building safe_run"

cargo build --release

SAFE_RUN_PATH=$(pwd)/safe_run/target/debug/
SAFE_SETUP_PATH=$(pwd)/safe_run/
SAFE_GIT_SH=$(pwd)/safe_run/src/

echo "SAFE_RUN_PATH: $SAFE_RUN_PATH"
echo "SAFE_SETUP_PATH: $SAFE_SETUP_PATH"
echo "SAFE_GIT_SH: $SAFE_GIT_SH"

echo "OS: $OSTYPE"


if [[ $OSTYPE == "linux" ]]; then
    echo "Linux detected"

    echo "[->] Adding safe_run PATH"
    PATH=$SAFE_RUN_PATH:$PATH

    echo "[->] Adding setup_safe_run.sh PATH"
    PATH=$SAFE_SETUP_PATH:$PATH
elif [[ $OSTYPE == "msys" ]]; then
    echo "Windows detected"

    echo "[->] Adding safe_run PATH"
    export PATH=$PATH:$SAFE_RUN_PATH

    echo "[->] Adding setup_safe_run.sh PATH"
    export PATH=$PATH:$SAFE_SETUP_PATH
else
    echo "OS not supported trying same as for linux"
    echo "[->] Adding safe_run PATH"
    PATH=$SAFE_RUN_PATH:$PATH

    echo "[->] Adding setup_safe_run.sh PATH"
    PATH=$SAFE_SETUP_PATH:$PATH
fi

echo $PATH

read