use regex::Regex;
use crate::{Config, PatternVS};

/// Function to find matches in a file
/// # Arguments
/// * `config` - Configuration for the search
/// * `file` - path to the file to search
/// * `PatternVS` - struct list of patterns to search
/// # Returns
/// * `Result` - struct of lists of matches
/// # Example
/// ```
/// Result {
///    matches: vec![vec![("pattern1","line"),("pattern1","line")],vec![("pattern2","line"),("pattern2","line")]],...]
/// }
/// ```
pub fn find_matches(config: &Config, file: &str, patterns: &Vec<PatternVS>) -> crate::Result {
    // initialize a vector for all the matches
    let mut matches = Vec::new();
    let mut h = false;
    let mut m = false;
    let mut first_match = true;
    let mut written_lines = Vec::new();
    for pattern_ in patterns {
        let mut found = Vec::new();
        let pattern = &pattern_.pattern;
        let is_regex = pattern_.regex;

        if is_regex {
            let mut line_number = 1;
            let re = Regex::new(&pattern).unwrap();

            // if fast is not set, go through all the lines else just finds if it contains the pattern
            if !config.fast {
                for line in file.lines() {
                    if re.is_match(line) {
                        if first_match {
                            print!("\n");
                            first_match = false;
                        }
                        if config.show && !written_lines.contains(&line_number) {
                            let severity = &pattern_.severity;

                            let line = format!("\t{}", line.trim());
                            match severity {
                                1 => {
                                    h = true;
                                    println!("\x1b[0;31;1m\t   line: {}|{}\x1b[0m", line_number, line)
                                }
                                2 => {
                                    m = true;
                                    println!("\x1b[0;33;1m\t   line: {}|{}\x1b[0m", line_number, line)
                                }
                                _ => println!("\x1b[0;34;1m\t   line: {}|{}\x1b[0m", line_number, line),
                            }
                            // println!("{}: {}", line_number, line);
                        }
                        found.push(line_number.to_string());
                        written_lines.push(line_number);
                    }
                    line_number += 1;
                }
            } else {
                for line in file.lines() {
                    if re.is_match(line) {
                        found.push(line_number.to_string());
                        break;
                    }
                }
            }
            matches.push(found);
        } else {
            let mut line_number = 1;
            // if fast is not set, go through all the lines else just finds if it contains the pattern
            if !config.fast {
                for line in file.lines() {
                    if line.contains(&pattern_.pattern) {
                        if config.show && !written_lines.contains(&line_number){
                            // Check for severity of the pattern (with regex)
                            let severity = pattern_.severity;

                            let line = format!("\t{}", line.trim());
                            match severity {

                                1 => {
                                    h = true;
                                    println!("\x1b[0;31;1m\t   line: {}|{}\x1b[0m",line_number, line)
                                },
                                2 => {
                                    m = true;
                                    println!("\x1b[0;33;1m\t   line: {}|{}\x1b[0m",line_number, line)
                                },
                                _ => println!("\x1b[0;34;1m\t   line: {}|{}\x1b[0m",line_number, line),
                            }
                            // println!("{}: {}", line_number, line);
                        }
                        found.push(line_number.to_string());
                        written_lines.push(line_number);
                    }
                    line_number += 1;
                }
            }else {
                for line in file.lines() {
                    if line.contains(&pattern[1..]) {
                        found.push(line_number.to_string());
                        break;
                    }
                }
            }
            matches.push(found);
        }
    };
    //
    crate::Result {
        matches,
        high: h,
        mid: m,
    }
}

pub fn att(n: &usize) -> &'static str {
    let att: &str;
    if *n == 0 {
        att = "~ WOOOOOOOOOOO ~";
    } else if *n < 5 {
        att = ":)";
    } else if *n < 10 {
        att = ":|";
    } else if *n > 10 && *n < 20 {
        att = ":(";
    } else {
        att = " WAT IN THE GODS NAME HAVE YOU DONE ";
    }
    att
}
