mod var_type;
mod settings_var;
mod settings_group;
mod settings_master;
mod xml_parsing;
mod to_witcher_script;

use std::{fs::OpenOptions, io::{Read, Write}};

use clap::Parser;
use to_witcher_script::ToWitcherScript;


#[derive(Parser)]
#[clap(name = "TW3 Settings Framework Parser")]
#[clap(version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown version"))]
#[clap(about = "Parses a mod menu XML file and outputs witcher script code representing settings of this menu", long_about=None)]
struct CLI {
    /// Path to the menu xml file
    #[clap(short = 'f')]
    xml_file_path: String,

    /// WitcherScipt output file
    #[clap(short = 'o')]
    output_ws_file_path: String,

    /// Name to use for the settings master class
    #[clap(short = 'c')]
    settings_master_name: String,

    //TODO option to ignore groups with certain keywords (or with lack there of)
}

fn main() {
    let cli = CLI::parse();

    
    let xml_file = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(cli.xml_file_path.clone());

    let ws_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(cli.output_ws_file_path.clone());


    let mut xml_file = match xml_file {
        Ok(f) => f,
        Err(e) => {
            println!("Error opening menu xml file: {}", e);
            return;
        }
    };

    let mut ws_file = match ws_file {
        Ok(f) => f,
        Err(e) => {
            println!("Error creating witcher script output file: {}", e);
            return;
        }
    };

    //TODO validating settings master name



    let mut xml_text = String::new();
    if let Err(e) = xml_file.read_to_string(&mut xml_text) {
        println!("Error reading menu xml file: {}", e);
        return;
    };

    match xml_parsing::parse_settings_xml(xml_text, cli.settings_master_name.clone()) {
        Ok(master) => {
            let mut code = String::new();

            code += &master.ws_code_body();
            code += "\n";
            for group in master.groups {
                code += &group.ws_code_body();
                code += "\n";
            }

            if let Err(e) = ws_file.write_all(code.as_bytes()) {
                println!("Error writing witcher script output file: {}", e);
                return;
            }

            println!("Successfully parsed {} into {}", cli.xml_file_path, cli.output_ws_file_path);
        }
        Err(e) => {
            println!("Error parsing menu xml file: {}", e);
        }
    }
}
