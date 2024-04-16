# safeRun
<br>safeRun is a command-line program that goes through the given directory and checks files if they contain any sensitive data.
<br>How to use:<br>
build the program using cargo
`cargo build`<br>
And finally, run the .exe with desired flags <br>
(You can also use `cargo run -- flags`)

## setup
(currently not working)<br>
If you wish to run this program automatically before commit you can run setup.sh<br>
This will add a pre-commit hook that will run the program before every commit.<br>
NOTE: you must first build the program using cargo build and add it to path<br>

## patterns.txt
This file can be anywhere or have any name.<br>
There are 2 ways the program will search for patterns:<br>
1. using Regex:<br>
   In the patterns file, we start the line containing the regex expression with `$`<br>
   -> example: `$.*[0-9].*` (of cores the expression can be more complicated than this)<br>
2. strings:<br>
   If we have a phrase or only a string we want to find we start the line with `"`<br>
   -> example: `"lucid-sonar-123415`<br>

## ignore.txt
This file can be anywhere or have any name<br>
Just write what to ignore (see ignore.txt for reference)


## Available flags:<br>
`-p` or `--path` : specify the path to a directory.  !required<br>
`-r` or `--pattern` : specify the path to the file with patterns to look for.  !currently required <br>
`-i` or `--ignore` : specify the path to the file that includes which dirs or folders to ignore. !optional <br>
`-f` or `--fast` : flag::if used the program will only find the first match within a file. <br>
`-s` or `--show-lines` ; flag::if used the program will display lines in which the pattern occurs <br>
note: `-f` and `-s` cannot be used together (if both are used the program will ignore `-s`)<br>
`-h` or `--help` : show help message <br>

## Examples <br>
Let's have a file ignore.txt, patterns.txt, and a dir with our project: C:\user\project<br>
### Default use <br>
`cargo run -- -p C:\user\project -r patterns.txt`<br>
-- This will show us which files contain the patterns and on what lines.--<br><br>

### -i <br>
`cargo run -- -p C:\user\project -r patterns.txt -i ignore.txt`<br>
-- This will show us which files contain the patterns and on what lines AND will ignore all files/folders within ignore.txt .--<br><br>

### -f <br>
`cargo run -- -p C:\user\project -r patterns.txt -f`<br>
-- This will show us which files contain the patterns.--<br><br>

### -s <br>
`cargo run -- -p C:\user\project -r patterns.txt`<br>
-- This will show us which files contain the patterns and on what lines AND it will also display those lines.--<br><br>


# 
Possible patterns to include in patterns.txt:<br>
- API keys: `$[A-Za-z0-9]{32}`<br>
- IP addresses: `$.*((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}`<br>
- Auth tokens: `$[a-zA-Z0-9-_=]+\.[a-zA-Z0-9-_=]+\.?[a-zA-Z0-9-_=]*$`<br>
- Email addresses: `$[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}`<br>
<br>Some additional patterns:<br>
- URLs: `$https?://[a-zA-Z0-9-_.]+`<br>
- Phone numbers: `$\+?[0-9]{1,3}[\s-]?[0-9]{3}[\s-]?[0-9]{3}[\s-]?[0-9]{3}`<br>
- Credit card numbers: `$[0-9]{4}-[0-9]{4}-[0-9]{4}-[0-9]{4}`<br>
- Social Security numbers: `$[0-9]{3}-[0-9]{2}-[0-9]{4}`<br>
