#!/bin/bash
# A script to get filepaths of all changed files in a git commit,
# create a temp dir with the files and their directory structure,
# and print the path to the temp dir. Then runs the safeRun program
# with the temp dir as a flag argument. After successful execution,
# the temp dir is removed.

# Ensure the script exits on any error
set -e

# Get the path to the project folder
projectDir=$(pwd)

# Get the path to the temp dir
tempDir="${projectDir}/temp"

# Create the temp dir
mkdir -p "${tempDir}"

# Get the filepaths of all changed files in the commit
# git diff-tree --no-commit-id --name-only -r HEAD | while read filePath; do
git diff --name-only | while read filePath; do
    # Create the directory structure in the temp dir
#    dirPath="${tempDir}/$(dirname "${filePath}")"
#    mkdir -p "${dirPath}"
    # Copy the file to the temp dir, replicating its directory structure
    cp "${projectDir}/${filePath}" "${filePath}"
    echo "Copied ${filePath} to ${filePath}"
    echo "${filePath}"
done

# Run the safeRun program with the temp dir as a flag argument, and other files from the project folder
safe_run -p "${tempDir}" -s

## Remove the temp dir