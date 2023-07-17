mod var_type;
mod settings_var;
mod settings_group;
mod settings_master;
mod to_witcher_script;
mod cli;
mod utils;

use std::{fs::OpenOptions, io::{Read, Write}, path::{Path, PathBuf}};

use clap::Parser;
use cli::CLI;
use settings_master::SettingsMaster;

use crate::to_witcher_script::ToWitcherScript;


fn main() -> Result<(), String>{
    let cli = CLI::parse();

    let input_file_path = Path::new(&cli.xml_file_path);
    let xml_file = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(&input_file_path);
    
    let mut xml_file = match xml_file {
        Ok(f) => f,
        Err(e) => {
            return Err(format!("Error opening menu xml file: {}", e));
        }
    };

    let ws_path = match &cli.output_ws_file_path  {
        Some(path) => PathBuf::from(&path),
        None => input_file_path.with_extension("ws")
    };

    let ws_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&ws_path);

    let mut ws_file = match ws_file {
        Ok(f) => f,
        Err(e) => {
            return Err(format!("Error creating witcher script output file: {}", e));
        }
    };

    

    let mut xml_text = String::new();
    if let Err(e) = xml_file.read_to_string(&mut xml_text) {
        return Err(format!("Error reading menu xml file: {}", e));
    };

    match SettingsMaster::from_xml(xml_text, &cli) {
        Ok(master) => {
            let mut code = String::new();

            code += &master.ws_code_body();
            code += "\n";
            for group in master.groups {
                code += &group.ws_code_body();
                code += "\n";
            }

            if let Err(e) = ws_file.write_all(code.as_bytes()) {
                return Err(format!("Error writing witcher script output file: {}", e));
            }

            println!("Successfully parsed {} into {}", cli.xml_file_path, ws_path.to_str().unwrap_or(""));
        }
        Err(e) => {
            return Err(format!("Error parsing menu xml file: {}", e));
        }
    }

    return Ok(())
}
