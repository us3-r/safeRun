use std::{fmt::Arguments, future};
use clap::builder::Str;
use regex::Regex;
use serde_json::de::SliceRead;
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
pub fn find_matches(config: &Config, file: &str, patterns: &Vec<PatternVS>, color: bool) -> crate::Result {


    // initialize a vector for all the matches
    let mut matches = Vec::new();
    let mut h = false;
    let mut m = false;
    let mut first_match = true;
    let mut written_lines = Vec::new();
    for pattern_ in patterns {
        let mut found = Vec::new();
        let mut line_ctx:Vec<String> = Vec::new();

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
                                    custom_println(color, "0;31;1", format_args!("\tline: {}|{}", line_number, line));
                                }
                                2 => {
                                    m = true;
                                    custom_println(color, "0;33;1", format_args!("\tline: {}|{}", line_number, line));
                                }
                                _ => {
                                    custom_println(color, "0;34;1", format_args!("\tline: {}|{}", line_number, line));
                                },
                            }
                            // println!("{}: {}", line_number, line);
                        }
                        found.push(vec![line_number.to_string(), line.to_string()]);
                        written_lines.push(line_number);
                    }
                    line_number += 1;
                }
            } else {
                for line in file.lines() {
                    if re.is_match(line) {
                        found.push(vec![line_number.to_string(), line.to_string()]);
                        break;
                    }
                }
            }
            // print!("{:?}", &found);
            matches.push(found);
        } else {
            let mut line_number = 1;
            // if fast is not set, go through all the lines else just finds if it contains the pattern
            if !config.fast {
                for line in file.lines() {
                    if line.contains(&pattern_.pattern) {
                        if config.show && !written_lines.contains(&line_number){
                            let severity = pattern_.severity;

                            let line = format!("\t{}", line.trim());
                            match severity {
                                1 => {
                                    h = true;
                                    custom_println(color, "0;31;1", format_args!("\tline: {}|{}", line_number, line));
                                }
                                2 => {
                                    m = true;
                                    custom_println(color, "0;33;1", format_args!("\tline: {}|{}", line_number, line));
                                }
                                _ => {
                                    custom_println(color, "0;34;1", format_args!("\tline: {}|{}", line_number, line));
                                },
                            }
                        }
                        found.push(vec![line_number.to_string(), line.to_string()]);
                        written_lines.push(line_number);
                    }
                    line_number += 1;
                }
            }else {
                for line in file.lines() {
                    if line.contains(&pattern[1..]) {
                        found.push(vec![line_number.to_string(), line.to_string()]);
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

    /* mathes = [["28", "        responses = runner.get_form_responses(service, form_id)  # 1NiMZe6U3hThZM2a9rg2ZlochRh8DtrnutRVI-CLIeEI"]][][][][][][] (example) */
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


/// Custom function to print with/without color (replace print!())
/// ### Arguments
/// #### `color` - bool to print with color or not
/// #### `color_code` - color code to print with
/// #### `format` - stuff to be printed [example: println!("{} {}", a, b) -> format_args!("{} {}", a, b)]
/// 
pub fn custom_print(color: bool, color_code: &str, format: Arguments){
    if color {
        print!("\x1b[{}m{}\x1b[0m", color_code, format);
    } else {
        print!("{}", format);
    }
}

/// Custom function to print with/without color (replace println!())
/// ### Arguments
/// #### `color` - bool to print with color or not
/// #### `color_code` - color code to print with
/// #### `format` - stuff to be printed [example: println!("{} {}", a, b) -> format_args!("{} {}", a, b)]
/// 
pub fn custom_println(color: bool, color_code: &str, format: Arguments){
    if color {
        println!("\x1b[{}m{}\x1b[0m", color_code, format);
    } else {
        println!("{}", format);
    }
}