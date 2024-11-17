use utils_rw::{custom_print, custom_println, ends_with_blank_line};
use clap::Parser;
use std::{
    fs,
    io::{self, Write},
    process
};
use structs::{Config, PatternVS};
use walkdir::WalkDir;

mod structs;
mod wsj;
mod utils_rw;


///Program to search for patterns in files
#[derive(Parser, Debug)]
#[command(name = "safeRun")]
#[command(version = "1.2")]
#[command(author = "us3-r")]
#[command(
    about = "Searches for phrases in files to find any sensitive data you might have left in your code"
)]
struct Args {
    /// Sets the path to search
    #[arg(short, long, required = true)]
    path: String,

    /// Path to settings.json file
    #[arg(short = 's', long, required = true)]
    settings: String,

    /// Only checks if patterns are present (not where)
    #[arg(short, long, default_value = "false")]
    fast: bool,

    /// prints the lines where the patterns are found
    #[arg(short = 'l', long, default_value = "false")]
    show_lines: bool,
}

fn main() {
    let mut found_pattern = 0;
    let config = parse_args();

    let settings = utils_rw::load_settings(&config.settings);
    let patterns: Vec<PatternVS> = utils_rw::load_patterns(&settings);
    let vector_for_report_whit_all = Vec::new();


    let ignore_list = &settings.ignore;
    let project_path = if config.path == "o" {
        &settings.project_path
    } else {
        &config.path
    };
    let color = settings.run_settings.color_output;
    let mut appearance = 0;

    for entry in WalkDir::new(&project_path) {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.display().to_string();
        let contains = ignore_list.iter().any(|i| path_str.contains(i.as_str()));
        if contains {
            continue;
        } else {
            if path.is_file() {
                let mut line_number = 1;
                let mut first_pattern = true;
                let ends_wbl = utils_rw::ends_with_blank_line(&path_str);

                let file = match fs::read(path) {
                    Ok(content) => {
                        let val = String::from_utf8_lossy(&content).to_string();
                        val
                    }
                    Err(e) => {
                        panic!("Error reading file: {}", e)
                    }
                };

                io::stdout().flush().unwrap();
                custom_print(color, "0;33;1", format_args!("\n\n{}", path.display()));

                for line in file.lines(){
                    if let Some(result) = utils_rw::find_matches(line, &patterns){
                        if first_pattern {
                            if result.severity_ == 1 {
                                found_pattern += 1;
                            }
                            custom_println(color, "0;31;1", 
                            format_args!("\n> !!! Patterns found !!! <"));
                            if settings.run_settings.show_patterns {
                                custom_println(color, "0;31;1", 
                                format_args!("| Pattern...{} [{}]", result.pattern_, result.severity_));
                            }
                            custom_println(color, "0;37;1", 
                            format_args!("+ |{}| {}", &line_number, &line));
                            first_pattern = false;
                        } else {
                            if settings.run_settings.show_patterns {
                                custom_println(color, "0;31;1", 
                                format_args!("| Pattern...{} [{}]",result.pattern_, result.severity_));
                            }
                            custom_println(color, "0;37;1", 
                            format_args!("+ |{}| {}", &line_number, &line));
                        }
                        appearance += 1;
                        line_number += 1;
                    }else{
                        line_number += 1;
                        continue;
                    }
                }
                if first_pattern{
                    custom_print(color, "0;32;1", format_args!("\t OK"));
                    if ends_wbl {
                        custom_print(color, "0;32;1", format_args!(" (ends with blank line)"));
                    } else {
                        custom_print(color, "0;31;1", format_args!(" (Does not end with blank line)"));
                    }
                } else {
                    if ends_wbl{
                        custom_println(color, "0;31;1", format_args!("\n(ends with blank line)"));
                    } else {
                        custom_println(color, "0;31;1", format_args!("\n(Does not end with blank line)"));
                    }

                }
            }
        }
    //     make so a sh script will fail if found_pattern > 0
        if found_pattern > 0 {
            custom_println(color, "0;31;1", format_args!("\n\nFound {} matches ... {}", appearance, utils_rw::att(&appearance)));
            process::exit(1);
        }
    }
    custom_println(
        color,
        "0;35;1",
        format_args!("\n+{:-<width$}+", "", width = 26 + project_path.len()),
    );
    custom_println(
        color,
        "0;35;1",
        format_args!(
            "Found {} matches ... {}",
            appearance,
            utils_rw::att(&appearance)
        ),
    );
    custom_println(
        color,
        "0;35;1",
        format_args!("+{:-<width$}+", "", width = 26 + project_path.len()),
    );

    // println!("{:?}", vector_for_report_whit_all);
    // TODO: need to implement the report
    if settings.run_settings.write_report {
        wsj::init_report(color, settings.report_settings, vector_for_report_whit_all);
    }
   

}

/// Parse command line arguments
fn parse_args() -> Config {
    let args = Args::parse();
    let path = args.path;
    let settings = args.settings;
    let fast = args.fast;
    let show = args.show_lines;
    // return Config (do not use ';' after the expression you want to return)

    Config {
        path: path.to_string(),
        settings: settings.to_string(),
        fast,
        show,
    }
}
