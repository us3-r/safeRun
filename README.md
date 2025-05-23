# safeRun

safeRun is a command-line program that scans a given directory for sensitive data patterns in files.

## How to Use

1. Build the program using Cargo:
   ```bash
   cargo build
   ```
And finally, run the .exe with desired flags <br>
(You can also use `cargo run -- flags`)


### Use as a pre-commit hook
If you wish to run this program automatically before commit, you can do it like so:<br>
1. build the program ```cargo build --release```<br>
2. run ```cargo install .``` (this will add safe_run.exe to path)<br>
3. in .\git\hooks create a file named pre-commit (without extension)<br>
4. add the code from ```pre-commit-sample``` to .git/hooks/pre-commit<br>
! If it doesn't work, check that the ```pre-commit``` file has no extension<br>
! and has correct permissions (it should be executable)<br>
5. if it still doesn't work try ChatGpt 🤝

## SETUP
(currently not working)<br>
If you wish to run this program automatically before commit, you can run setup.sh<br>
This will add a pre-commit hook that will run the program before every commit.<br>
NOTE: you must first build the program using cargo build and add it to path<br>


## PATTERNS block
```json
{
   "patterns": {
      "severity": {
         "h": [],
         "m": [],
         "l": []
      }
   }
}
```
<br> it is used to store patterns you want to search for.<br>
Into each severity list you add a pattern block which looks like so:<br>

```json
{
    "pattern": "[A-Za-z0-9]{128}" , 
    "comment": "check for SHA-512 hash", 
    "regex": "true"
}
```

Where:<br>
``pattern`` is a regex pattern or a string you want to search for.<br>
``comment`` is a comment that will be displayed when the pattern is found.<br>
``regex`` is a boolean value that tells the program if the pattern is a regex pattern or a string.<br>

## IGNORE block

```json
{
   "ignore": []
}
``` 
<br> it is used to store files or paths you do not want to search through.<br>
You can add files or directories to the ignored block in the settings.json file as if you would to .gitignore<br>


## Available flags:<br>
`-p` or `--path` : specify the path to a directory.  !required<br>
`-s` or `--settings`: path to settings.json <br>
`-f` or `--fast` : flag::if used the program will only find the first match within a file. <br>
`-l` or `--show-lines` ; flag::if used the program will display lines in which the pattern occurs <br>
note: `-f` and `-s` cannot be used together (if both are used the program will ignore `-s`)<br>
`-h` or `--help` : show help message <br>

## Examples <br>
Lets have a dir with our project be: C:\user\project<br>
#### Default use <br>
`cargo run -- -p C:\user\project -s settings.json`<br>
This will show which files contains the patterns and on what lines<br>
#### Fast Search <br>
`cargo run -- -p C:\user\project -s settings.json -f`<br>
This will show which files contain the patterns but only the first match in each file.<br>
#### Show lines <br>
`cargo run -- -p C:\user\project -s settings.json -l`<br>
This will show which files contain the patterns, on what lines, and display those lines. <br>


## REPORTS
TODO: currently under development<br>

#
Possible patterns to include in patterns.txt:<br>
- API keys: `$[A-Za-z0-9]{32}`<br>
- IP addresses: `$.*((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}`<br>
- Auth tokens: `$[a-zA-Z0-9-_=]+\.[a-zA-Z0-9-_=]+\.?[a-zA-Z0-9-_=]*$`<br>
- Email addresses: `$[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}`<br>
- URLs: `$https?://[a-zA-Z0-9-_.]+`<br>
- Phone numbers: `$\+?[0-9]{1,3}[\s-]?[0-9]{3}[\s-]?[0-9]{3}[\s-]?[0-9]{3}`<br>
- Credit card numbers: `$[0-9]{4}-[0-9]{4}-[0-9]{4}-[0-9]{4}`<br>
- Social Security numbers: `$[0-9]{3}-[0-9]{2}-[0-9]{4}`<br>
