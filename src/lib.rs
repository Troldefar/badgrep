use std::fs;
use std::env;
use std::error::Error;

pub struct FileConfig {
    pub query: String,
    pub file: String,
    pub case_insensitive: bool
}

impl FileConfig {
    pub fn build(args: &[String]) -> Result<FileConfig, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments was provided. Must be include query and file");
        }

        let query = args[1].clone();
        let file  = args[2].clone();
        let case_insensitive = env::var("CASE_INSENSITIVE").is_ok();

        Ok(FileConfig { 
            query,
            file,
            case_insensitive
        })
    }  
}

pub fn run(file_config: FileConfig) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_config.file)?;

    let results = if file_config.case_insensitive {
        search_case_insensitive(&file_config.query, &contents)
    } else {
        search(&file_config.query, &contents) 
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(
    query: &str, 
    contents: &'a str
) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

fn search_case_insensitive<'a>(
    query: &str, 
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
