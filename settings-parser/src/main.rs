mod var_type;
mod settings_var;
mod settings_group;
mod settings_master;
mod xml_parsing;
mod to_witcher_script;

use std::{fs::OpenOptions, io::{Read, Write}, path::{Path, PathBuf}};

use clap::Parser;
use to_witcher_script::ToWitcherScript;


#[derive(Parser)]
#[clap(name = "TW3 Settings Framework Parser")]
#[clap(version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown version"))]
#[clap(about = "Parses a mod menu XML file and outputs witcher script code representing settings of this menu", long_about=None)]
struct CLI {
    /// Path to the menu xml file
    #[clap(long = "file", short = 'f')]
    xml_file_path: String,

    /// Name to use for the settings master class
    #[clap(long = "master", short = 'm')]
    settings_master_name: String,

    /// Path of the WitcherScipt output file, by default it's made from the menu xml file name in the same directory
    #[clap(long = "output", short = 'o')]
    output_ws_file_path: Option<String>,

    /// Prefix to omit from groups and variables, case sensitive
    #[clap(long)]
    omit_prefix: Option<String>,
}

fn main() {
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
            println!("Error opening menu xml file: {}", e);
            return;
        }
    };

    let ws_path = match cli.output_ws_file_path  {
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
            println!("Error creating witcher script output file: {}", e);
            return;
        }
    };

    

    let mut xml_text = String::new();
    if let Err(e) = xml_file.read_to_string(&mut xml_text) {
        println!("Error reading menu xml file: {}", e);
        return;
    };

    match xml_parsing::parse_settings_xml(xml_text, cli.settings_master_name, cli.omit_prefix) {
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

            println!("Successfully parsed {} into {}", cli.xml_file_path, ws_path.to_str().unwrap_or(""));
        }
        Err(e) => {
            println!("Error parsing menu xml file: {}", e);
        }
    }
}
