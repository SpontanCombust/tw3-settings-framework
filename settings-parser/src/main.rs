mod settings_var_type;
mod settings_var;
mod settings_group;
mod settings_master;
mod cli;
mod utils;
mod traits;
mod indented_document;
mod xml;
mod settings_enum;

use std::{fs::OpenOptions, io::{Read, Write}, path::{Path, PathBuf}};

use clap::Parser;
use cli::CLI;
use roxmltree::Document;
use settings_master::SettingsMaster;
use xml::user_config::UserConfig;

use crate::{traits::{WitcherScript, WitcherScriptTypeDef}, utils::validate_name};


fn main() -> Result<(), String>{
    let cli = CLI::parse();

    if let Err(err) = validate_name(&cli.settings_master_name) {
        return Err(format!("Invalid settings master name: {}", err));
    }

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

    let doc = match Document::parse(&xml_text) {
        Ok(doc) => doc,
        Err(err) => {
            return Err(format!("Document parsing error: {}", err))
        }
    };

    match UserConfig::try_from(&doc) {
        Ok(user_config) => {
            let settings_master = SettingsMaster::from(&user_config, &cli)?;

            let mut buffer = WitcherScript::new();

            buffer.push_line(&format!("// Code generated using Mod Settings Framework v{} by SpontanCombust & Aeltoth", option_env!("CARGO_PKG_VERSION").unwrap()))
                  .new_line();

            settings_master.ws_type_definition(&mut buffer);

            // clear file content if there is any
            ws_file.set_len(0).unwrap();
            
            if let Err(e) = ws_file.write_all(buffer.text.as_bytes()) {
                return Err(format!("Error writing witcher script output file: {}", e));
            }
        },
        Err(err) => {
            return Err(format!("Error parsing menu xml file: {}", err));
        }
    }

    println!("Successfully parsed {} into {}", cli.xml_file_path, ws_path.to_str().unwrap_or(""));
    return Ok(())
}
