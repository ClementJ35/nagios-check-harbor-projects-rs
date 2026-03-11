use std::{collections::HashMap, usize};
use clap::Parser;
use ureq::{self, Error};
use regex::Regex;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Url to check
    #[arg(short, long)]
    url: String,

    /// Warning percentage used from project quota
    #[arg(short, long, default_value_t = 80)]
    warning: u8,
    
    /// Critical percentage used from project quota
    #[arg(short, long, default_value_t = 90)]
    critical: u8,
}

// Convert scientific exp notation to int : 1.2345+e5 -> 123450
fn convert_scientific_notation_to_int(numeric_string: &String) -> u64 {

    // Initialize return
    let mut return_int: u64 = 0;

    // Cut prometheus style metrics 1.2334e+08
    let splitted_string: Vec<&str> = numeric_string.split("e+").collect();

    // Check if it was raw value or e+ style
    if splitted_string.len() == 2 {

        // Manipulate metrics to int:  1.2345e08 -> 12345 / 8
        let raw_value = splitted_string[0].replace(".", "");
        let exp_value = splitted_string[1].parse::<u8>().unwrap();

        // fill with zeroes if needed
        if raw_value.len() <= usize::from(exp_value) {

            let fill_number = &exp_value + 1 - raw_value.len() as u8;
            return_int = raw_value.parse::<u64>().unwrap() * u64::pow(10, fill_number as u32) ;

        } else if raw_value.len() == usize::from(exp_value) + 1 {
            return_int = raw_value.parse::<u64>().unwrap();
        }
    }

    return return_int;
}

fn main() -> Result<(), Error> {
    // Parser Args
    let args: Args = Args::parse();
    let url: String = args.url;
    let warning: u8 = args.warning;
    let critical: u8 = args.critical;

    // placeholder for criticals and warnings
    let mut critical_list: Vec<String> = [].to_vec();
    let mut warning_list: Vec<String> = [].to_vec();

    // placeholder to send result
    let mut resut_line: String = "".to_string();
    let mut project_line: String = "".to_string();

    
    // Definition of the http get
    let body = ureq::get(url)
        .call()?
        .body_mut()
        .read_to_string()?;

    // Regex to extract only project related lines then vectorize
    let re_projects = Regex::new(r".*project_name=.*\n").unwrap();
    let project_lines: Vec<&str> = re_projects.find_iter(&body).map(|m|m.as_str()).collect();

    // Hashmap to store results ordering by projects
    let mut projects: HashMap<String, HashMap<String, String> > = HashMap::new();

    // Regex to extract data from metrics lines
    let re_project_name = Regex::new(
        r#"harbor_(?<subject>\w+)\{(?<artifact_type>.*)project_name="(?<name>[\w\.-]+)".*\} (?<value>.*)"#)
        .unwrap();

    // Line by line extraction
    for line in project_lines {

        // Extraction and storing into variables
        let project_parser = re_project_name.captures(line).unwrap();
        let project_name: String = project_parser["name"].to_string();
        let subject: String = project_parser["subject"].to_string();
        let value: String = project_parser["value"].to_string();

        // Building the hashmap
        if projects.contains_key(project_name.as_str()) {

            let temp_hash = projects.get_mut(&project_name).unwrap();
            temp_hash.insert(subject, value);
        
        } else {

            let temp_hash: HashMap<String, String> = HashMap::new();
            projects.insert(project_name, temp_hash);

        }
    }

    // Creating output from hashmap
    for (hash_project_name, hash_metrics) in projects {

        // Convert metrics to int
        let used_quota: u64 = convert_scientific_notation_to_int(&hash_metrics["project_quota_usage_byte"]);
        let total_quota: u64 = convert_scientific_notation_to_int(&hash_metrics["project_quota_byte"]);

        // Compute usage percentage
        let percentage_quota: u64 = used_quota * 100 / total_quota;
        
        // Create NRPE style string
        project_line += &format!("{}={};{warning};{critical};0;100", &hash_project_name, &percentage_quota);

        // Check if we need to alert as crit or warn
        if percentage_quota >= critical as u64 {

            critical_list.push(hash_project_name);

        } else if percentage_quota >= warning as u64 {

            warning_list.push(hash_project_name);

        }

    }



    println!("{project_line}");
    Ok(())
}
