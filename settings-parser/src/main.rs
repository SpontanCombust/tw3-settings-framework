mod var_type;
mod settings_var;
mod settings_group;
mod settings_master;
mod cli;
mod utils;
mod traits;
mod indented_document;

use std::{fs::OpenOptions, io::{Read, Write}, path::{Path, PathBuf}};

use clap::Parser;
use cli::CLI;
use roxmltree::Document;
use settings_master::SettingsMaster;
use traits::FromXmlNode;

use crate::traits::{ToWitcherScriptType, WitcherScript};


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
            //TODO these errors might be to verbose, the last message is the only important one
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

    let doc = match Document::parse(&xml_text) {
        Ok(doc) => doc,
        Err(err) => {
            return Err(format!("Document parsing error: {}", err))
        }
    };

    if let Some(root_node) = doc.descendants().find(|n| n.has_tag_name("UserConfig")) {
        match SettingsMaster::from_xml_node(&root_node, &cli) {
            Ok(master) => {
                let master = master.unwrap();

                let mut ws = WitcherScript::new();

                ws.push_line(&format!("// Code generated using Mod Settings Framework v{} by SpontanCombust & Aeltoth", option_env!("CARGO_PKG_VERSION").unwrap()))
                  .new_line();
    
                master.ws_type_definition(&mut ws);
                ws.new_line();
    
                for group in master.groups {
                    if group.ws_type_definition(&mut ws) {
                        ws.new_line();
                    }
    
                    for var in group.vars {
                        if var.ws_type_definition(&mut ws) {
                            ws.new_line();
                        }
                    }
                }
    
                if let Err(e) = ws_file.write_all(ws.text.as_bytes()) {
                    return Err(format!("Error writing witcher script output file: {}", e));
                }
    
                println!("Successfully parsed {} into {}", cli.xml_file_path, ws_path.to_str().unwrap_or(""));
            }
            Err(e) => {
                return Err(format!("Error parsing menu xml file: {}", e));
            }
        }
    }

    return Ok(())
}
