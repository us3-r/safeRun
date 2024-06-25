#!/bin/bash

# Determine the shell configuration file
if [[ $SHELL == */bash ]]; then
    CONFIG_FILE=~/.bash_profile
elif [[ $SHELL == */zsh ]]; then
    CONFIG_FILE=~/.zshrc
else
    echo "Unsupported shell. Please manually add the paths to your shell configuration file."
    exit 1
fi

# Define the paths
SAFE_RUN_PATH=$(pwd)/safe_run/target/debug/
SAFE_SETUP=$(pwd)/safe_run/setup_safe_run.sh
SAFE_GIT_SH=$(pwd)/safe_run/src/
#
## Add the paths to the shell configuration file
#{
#    echo ""
#    echo "# Add safe_run paths"
#    echo "SAFE_RUN_PATH=$SAFE_RUN_PATH"
#    echo "SAFE_SETUP=$SAFE_SETUP"
#    echo "SAFE_GIT_SH=$SAFE_GIT_SH"
#    echo "export PATH=\$SAFE_RUN_PATH:\$PATH"
#    echo "export PATH=\$SAFE_SETUP:\$PATH"
#    echo "export PATH=\$SAFE_GIT_SH:\$PATH"
#} >> $CONFIG_FILE

# Define the markers for the added paths
START_MARKER="# Add safe_run paths"
END_MARKER="export PATH=\$SAFE_GIT_SH:\$PATH"

# Remove the lines between the markers
sed -i "/$START_MARKER/,/$END_MARKER/d" $CONFIG_FILE


# Source the configuration file to apply the changes
# shellcheck disable=SC1090
source $CONFIG_FILE

echo "Paths have been added to $CONFIG_FILE and applied to the current session."
echo $PATH
read