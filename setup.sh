#!/bin/bash

# 3. Add safe_run to PATH
# Assuming safe_run executable is in the current directory

# UNCOMMENT THE FOLLOWING IF USING UNIX
#SAFE_RUN_PATH=$(pwd)/safe_run
#echo "Adding safe_run to PATH for this session."
#export PATH=$PATH:$SAFE_RUN_PATH
## To permanently add it to PATH, append the export command to .bashrc or .bash_profile
#echo "export PATH=\$PATH:$SAFE_RUN_PATH" >> "$HOME/.bashrc"


#Copy gitInt.sh to .git/hooks and rename it to pre-commit
echo "Copying gitInt.sh to .git/hooks/pre-commit"
cp src/gitInt.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
echo "pre-commit hook has been set."

echo "Script execution completed."
