use clap::Parser;

#[derive(Parser)]
#[clap(name = "TW3 Settings Framework Parser")]
#[clap(version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown version"))]
#[clap(about = "Parses a mod menu XML file and outputs witcher script code representing settings of this menu", long_about=None)]
pub struct CLI {
    /// Path to the menu xml file
    #[clap(long = "file", short = 'f')]
    pub xml_file_path: String,

    /// Name to use for the settings master class
    #[clap(long = "master", short = 'm')]
    pub settings_master_name: String,

    /// Path of the WitcherScipt output file, by default it's made from the menu xml file name in the same directory
    #[clap(long = "output", short = 'o')]
    pub output_ws_file_path: Option<String>,

    /// Prefix to omit from groups and variables, case sensitive
    #[clap(long)]
    pub omit_prefix: Option<String>
}