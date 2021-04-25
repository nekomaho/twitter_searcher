use std::fs;

use yaml_rust::YamlLoader;

pub struct Config {
    pub screen_name: String,
    pub extract_keyword: String,
    pub output_lines: String,
    pub extract_lines: Vec<String>
}

const CONFIG_FILE: &str = "config.yml";

impl Config {
    pub fn new() -> Result<Self, i32>{
        let config_content = match fs::read_to_string(CONFIG_FILE) {
            Ok(content) => content,
            Err(_) => {
                println!("can not load config.yml");
                return Err(1);
            }
        };

        let content = match YamlLoader::load_from_str(&config_content) {
            Ok(result) => result,
            Err(_) => {
                println!("can not parse config.yml");
                return Err(1);
            }
        };
        
        let config = &content[0];

        let mut extract_lines : Vec<String> = Vec::new();

        for config_extract_line in config["extract_lines"].as_vec().unwrap() {
            extract_lines.push(config_extract_line.as_str().unwrap().to_string());
        }

        Ok(Config {
            screen_name: config["screen_name"].as_str().unwrap().to_string(),
            extract_keyword: config["extract_keyword"].as_str().unwrap().to_string(),
            output_lines: config["output_lines"].as_i64().unwrap().to_string(),
            extract_lines: extract_lines,
        })
    }
}