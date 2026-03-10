use std::{collections::{HashMap, hash_map}, string};
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

fn main() -> Result<(), Error> {
    // Parser Args
    let args: Args = Args::parse();
    let url: String = args.url;
    let warning: u8 = args.warning;
    let critical: u8 = args.critical;

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

        // Building of the hashmap
        if projects.contains_key(project_name.as_str()) {

            let temp_hash = projects.get_mut(&project_name).unwrap();
            temp_hash.insert(subject, value);
        
        } else {

            let temp_hash: HashMap<String, String> = HashMap::new();
            projects.insert(project_name, temp_hash);

        }
    }

    println!("{:?}",projects);

    Ok(())
}
