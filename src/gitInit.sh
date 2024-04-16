#!/bin/bash

# A script to get filepaths of all changed files in a git commit,
# create a temp dir with the files and their directory structure,
# and print the path to the temp dir. Then runs the safeRun program
# with the temp dir as a flag argument. After successful execution,
# the temp dir is removed.


# shellcheck disable=SC2162
# shellcheck disable=SC2242



# Ensure the script exits on any error
set -e

# Get the path to the project folder
projectDir=$(pwd)
echo "Project directory: ${projectDir}"

# Get the path to the temp dir
tempDir="${projectDir}/temp"

# Create the temp dir
mkdir -p "${tempDir}"

# Get the file paths of all changed files in the commit
# git diff-tree --no-commit-id --name-only -r HEAD | while read filePath; do
git diff --name-only -r HEAD | while read filePath; do
    # Create the directory structure in the temp dir
    echo "Creating directory structure for ${filePath}"
    dirPath="${tempDir}/$(dirname "${filePath}")"
    mkdir -p "${dirPath}"

    # Copy the file to the temp dir, replicating its directory structure
    cp "${projectDir}/${filePath}" "${dirPath}"
done

# Run the safeRun program with the temp dir as a flag argument, and other files from the project folder
out=$(safe_run --path "${tempDir}" -p patterns.txt -i ignore.txt -s | tee /dev/tty)

# Remove the temp dir
rm -rf "${tempDir}"
# Check the safeRunOutput for the summary line and extract the number of matches
matches=$(echo "${out}" | grep -oP 'Done , found \K\d+(?= matches)')

# Exit with a non-zero status if the number of matches is greater than 0
if [ "$matches" -gt 0 ]; then
    echo "Commit failed: found ${matches} matches."
    exit 1
else
    echo "Commit succeeded: no matches found."
fi

exit 0