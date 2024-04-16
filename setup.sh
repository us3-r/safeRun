#!/bin/bash

# UNCOMMENT THE FOLLOWING IF USING UNIX
#SAFE_RUN_PATH=$(pwd)/safe_run
#echo "Adding safe_run to PATH for this session."
#export PATH=$PATH:$SAFE_RUN_PATH
## To permanently add it to PATH, append the export command to .bashrc or .bash_profile
#echo "export PATH=\$PATH:$SAFE_RUN_PATH" >> "$HOME/.bashrc"


# TODO: currently only works for adding to .git in working dir, but should be global soooo
# - need to add path prefix to src/gitInit.sh so it could be accessed from anywhere
# - also need to add code to add save_run to PATH


#Copy gitInit.sh to .git/hooks and rename it to pre-commit
echo "Copying gitInit.sh to .git/hooks/pre-commit"
if [ -e .git/hooks/pre-commit ]; then
    echo "pre-commit hook already exists. Overwriting..."
    chmod +x .git/hooks/pre-commit
    rm .git/hooks/pre-commit
fi
cp src/gitInit.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit

echo "pre-commit hook has been set."
echo "Script execution completed."
read