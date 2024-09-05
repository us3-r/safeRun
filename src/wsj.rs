/*

settings.json :

"report_settings": {
        "report_path": "C:\\Users\\gfhyt\\CLionProjects\\rust\\safeRun",
        "report_title": "Test-title",
        "report_params": {
            "type_": "txt", <- format of report [html, md, txt]
            "include_": "all", <- what to include about found patterns [all, line, pattern, severity]
            "exclude_": "none", <- what to exclude about found patterns [all, line, pattern, severity]
            "caps_": false, <- write report in caps
            "filename_as_head": true <- idk what this is
        }
    },

*/


use crate::structs;
use std::fs::File;
use std::io::Write;
use crate::utils_rw::custom_println;

// mby should be write report? we inint and also write all the data so no need for 2 functions


/// Initializes the report creation process
/// ### Arguments
/// #### `paracolor`: bool - if true, colored output is enabled (for terminal)
/// #### `input_params`: structs::ReportSettings - settings for the report
/// #### `data`: Vec<Vec<String>> - data to be written in the report
/// #### `data` should look something like this:
///```
/// [
///    "<filename>",       (ind0
///    "<line number>",    (ind1
///    "<line content>",   (ind2
///    "<pattern>",        (ind3
///    "<severity>"        (ind4
///    "<blank_line>"      (ind5
///]
/// ```
/// 
pub fn init_report(color: bool, input_params: structs::ReportSettings, data: Vec<Vec<String>>) {
    custom_println(color, "0;33;1", format_args!("Creating report ..."));

    let path_ = input_params.report_path;
    let title_ = input_params.report_title;
    let type_ = input_params.report_params.type_;
    let exclude_ = input_params.report_params.exclude_;
    let caps_ = input_params.report_params.caps_;
    
    custom_println(color, "0;33;1", format_args!(
        "| Report will be made with the following data:\n+ Path  :\t{}\n+ Title :\t{}\n+ Type  :\t{}",
        path_, title_, type_
    ));
    
    
    
    let file_path_and_name = format!("{}\\{}.{}", path_, title_, type_);

    let mut file = File::create(&file_path_and_name).unwrap();

    match type_.as_str() {
        "html" => {
            let html = format!("<html>\n<head>\n<title>{}</title>\n</head>\n<body style=\"font-family: \'Courier New\'\">\n", title_);
            file.write_all(html.as_bytes()).unwrap();
            for data_ in data {
                write_as_report_html(&mut file, data_, exclude_.clone(), caps_);
            }
        }
        "md" => {
            let md = format!("# {}\n", title_);
            file.write_all(md.as_bytes()).unwrap();
            for data_ in data {
                write_as_report_md(&mut file, data_, exclude_.clone(), caps_);
            }
        }
        "json" => {
            let json = "{";
            file.write_all(json.as_bytes()).unwrap();
            for data_ in data {
                write_as_report_json(&mut file, data_, exclude_.clone(), caps_);
            }
        }
        _ => {
            let txt = format!("{}\n", title_);
            file.write_all(txt.as_bytes()).unwrap();
            for data_ in data {
                write_as_report_txt(&mut file, data_, exclude_.clone(), caps_);
            }
        }
    }
}

/*
data should look something like this:

[
    "<filename>",       0
    "<line number>",    1
    "<line content>",   2
    "<pattern>",        3
    "<severity>"        4
    "<blank_line>"      5
]

*/

/// Writes the data to the report file in txt format
/// ### Arguments
/// #### `file`: &mut File - the file to write to
/// #### `data`: Vec<String> - the data to be written
/// #### `exc`: String - what to exclude from the report
/// #### `caps`: bool - if true, the report will be written in caps
/// 
fn write_as_report_txt(file: &mut File, data: Vec<String>, exc: String, caps: bool) {
    let context: String;
    match exc.as_str() {                    
        "none" => {
            context = format!("In file: {} at line {}; ctx--> {}; LVL: {} -pattern-> {};\nEnds with blank? {}\n", data[0], data[1], data[2], data[4], data[3], data[5]);
        }
        "line" => {
            context = format!("In file: {}; LVL: {} -pattern-> {}\n", data[0], data[4], data[3]);
        }
        "pattern" => {
            context = format!("In file: {} at line {};ctx--> {}; LVL: {}\n", data[0], data[1], data[2], data[4]);
        }
        "severity" => {
            context = format!("In file: {} at line {};ctx--> {}; -pattern-> {}\n", data[0], data[1], data[2], data[3]);

        }
        _ => {
            context = format!("Error in report settings");
        }       
    }
    if caps {
        file.write_all(context.to_uppercase().as_bytes()).unwrap();
    } else{
        file.write_all(context.as_bytes()).unwrap();
    }
}


/// Writes the data to the report file in md format
/// ### Arguments
/// #### `file`: &mut File - the file to write to
/// #### `data`: Vec<String> - the data to be written
/// #### `exc`: String - what to exclude from the report
/// #### `caps`: bool - if true, the report will be written in caps
/// 
fn write_as_report_md(file: &mut File, data: Vec<String>, exc: String, caps: bool){
    let context: String;
    let split_name:Vec<&str> = data[0].split(".").collect();

    match exc.as_str() {                    
        "none" => {
            context = format!("\nIn file: ***{}***\n<br>\nLine: **{}**\n<br>\nPattern: **{}**\n<br>\nLVL: **{}**\n<br>\nLine context:\n```{}\n {}\n```\n<br>\n", data[0], data[1], data[3], data[4].to_uppercase(), split_name[1], data[2]);
        }
        "line" => {
            context = format!("\nIn file: ***{}***\n<br>\nPattern: **{}**\n<br>\nLVL: **{}**\n<br>\n", data[0],data[3], data[4].to_uppercase());
        }
        "pattern" => {
            context = format!("\nIn file: ***{}***\n<br>\nLine: **{}**\n<br>\nLVL: **{}**\n<br>\nLine context:\n```{}\n {}\n```\n<br>\n", data[0], data[1], data[4].to_uppercase(), split_name[1], data[2]);
        }
        "severity" => {
            context = format!("\nIn file: ***{}***\n<br>\nLine: **{}**\n<br>\nPattern: **{}**\n<br>\nLine context:\n```{}\n {}\n```\n<br>\n", data[0], data[1], data[3],split_name[1], data[2]);
        }
        _ => {
            context = format!("Error in report settings");
        }       
    }
    if caps {
        file.write_all(context.to_uppercase().as_bytes()).unwrap();
    } else{
        file.write_all(context.as_bytes()).unwrap();
    }
}

fn write_as_report_html(file: &mut File, data: Vec<String>, exc: String, caps: bool){
    let context: String;
    let split_name:Vec<&str> = data[0].split(".").collect();

    match exc.as_str() {
        "none" => {
            context = format!(
                "<p>In file: <strong>{}</strong><br>\n\
                Line: <strong>{}</strong><br>\n\
                Pattern: <strong>{}</strong><br>\n\
                LVL: <strong>{}</strong><br>\n\
                Line context:<br>\n\
                <pre style=\"background-color: #f5f5f5; padding: 10px; border-radius: 5px; font-family: 'Courier New', monospace;\">\n\
                <code>{}\n {}\n</code></pre><br>\n</p>",
                data[0], // File name
                data[1], // Line number
                data[3], // Pattern
                data[4].to_uppercase(), // Severity level (uppercased)
                split_name[1], // Some context name
                data[2] // Line context
            );
        }
        "line" => {
            context = format!(
                "<p>In file: <strong>{}</strong><br>\n\
                Pattern: <strong>{}</strong><br>\n\
                LVL: <strong>{}</strong><br>\n</p>",
                data[0], // File name
                data[3], // Pattern
                data[4].to_uppercase() // Severity level (uppercased)
            );
        }
        "pattern" => {
            context = format!(
                "<p>In file: <strong>{}</strong><br>\n\
                Line: <strong>{}</strong><br>\n\
                LVL: <strong>{}</strong><br>\n\
                Line context:<br>\n\
                <pre style=\"background-color: #f5f5f5; padding: 10px; border-radius: 5px; font-family: 'Courier New', monospace;\">\n\
                <code>{}\n {}\n</code></pre><br>\n</p>",
                data[0], // File name
                data[1], // Line number
                data[4].to_uppercase(), // Severity level (uppercased)
                split_name[1], // Some context name
                data[2] // Line context
            );
        }
        "severity" => {
            context = format!(
                "<p>In file: <strong>{}</strong><br>\n\
                Line: <strong>{}</strong><br>\n\
                Pattern: <strong>{}</strong><br>\n\
                Line context:<br>\n\
                <pre style=\"background-color: #f5f5f5; padding: 10px; border-radius: 5px; font-family: 'Courier New', monospace;\">\n\
                <code>{}\n {}\n</code></pre><br>\n</p>",
                data[0], // File name
                data[1], // Line number
                data[3], // Pattern
                split_name[1], // Some context name
                data[2] // Line context
            );
        }

        _ => {
            context = String::from("<p>Error in report settings</p>");
        }
    }
    if caps {
        file.write_all(context.to_uppercase().as_bytes()).unwrap();
    } else{
        file.write_all(context.as_bytes()).unwrap();
    }
    
}


/// Writes the data to the report file in json format
/// ### Arguments
/// #### `file`: &mut File - the file to write to
/// #### `data`: Vec<String> - the data to be written
/// #### `exc`: String - what to exclude from the report
/// #### `caps`: bool - if true, the report will be written in caps
/// 
fn write_as_report_json(file: &mut File, data: Vec<String>, exc: String, caps: bool){
    // only a place holder for now, can be rewritten to be more efficient and to clean <context string> before adding it to the file (strip of ", {,...)
    /*
        {    
            {
                "file": "<filename>",
                "line": "<line number>",
                "pattern": "<pattern>",
                "severity": "<severity>",
                "context": "<line content>"
            }
        }
    */

    let context: String;

    

    match exc.as_str() {
        "none" => {
            context = format!(
                "\t{{\n\t\t\"file\":\"{}\",\n\t\t\"line\":\"{}\",\n\t\t\"pattern\":\"{}\",\n\t\t\"severity\":\"{}\",\n\t\t\"context\":\"{}\"\n\t}},\n",
                data[0], data[1], data[3], data[4].to_uppercase(), data[2]
            );           
        }
        "line" => {
            context = format!(
                "\t{{\n\t\t\"file\":\"{}\",\n\t\t\"pattern\":\"{}\",\n\t\t\"severity\":\"{}\"\n\t}},\n",
                data[0], data[3], data[4].to_uppercase()
            );             
        }
        "pattern" => {
            context = format!(
                "\t{{\n\t\t\"file\":\"{}\",\n\t\t\"line\":\"{}\",\n\t\t\"severity\":\"{}\",\n\t\t\"context\":\"{}\"\n\t}},\n",
                data[0], data[1], data[4].to_uppercase(), data[2]
            );             
        }
        "severity" => {
            context = format!(
                "\t{{\n\t\t\"file\":\"{}\",\n\t\t\"line\":\"{}\",\n\t\t\"pattern\":\"{}\",\n\t\t\"context\":\"{}\"\n\t}},\n",
                data[0], data[1], data[3], data[2]
            );             
        }

        _ => {
            context = String::from("\t{\"error\":\"error\"},\n");
        }
    }
    if caps {
        file.write_all(context.to_uppercase().as_bytes()).unwrap();
    } else{
        file.write_all(context.as_bytes()).unwrap();
    }

}