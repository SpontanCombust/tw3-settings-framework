use clap::Parser;

#[derive(Parser)]
#[clap(name = "TW3 Settings Framework Parser")]
#[clap(version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown version"))]
#[clap(about = "Parses a mod menu XML file and outputs witcher script code representing settings of this menu", long_about=None)]
pub struct CLI {
    /// Path to the menu xml file. (REQUIRED)
    #[clap(long = "file", short = 'f', display_order=0)]
    pub xml_file_path: String,

    /// Name to use for the settings master class. (REQUIRED)
    #[clap(long = "master", short = 'm', display_order=1)]
    pub settings_master_name: String,

    /// Version of the mod in format. Format is arbitrary, but preferably a real number.
    #[clap(long, short = 'v', default_value="1.0", display_order=2)]
    pub mod_version: String,

    /// Path of the WitcherScipt output file, by default it's made from the menu xml file name in the same directory.
    #[clap(long = "output", short = 'o', display_order=3)]
    pub output_ws_file_path: Option<String>,


    /// Prefix to omit from groups and vars when generating code. Case sensitive.
    #[clap(long, display_order=4)]
    pub omit_prefix: Option<String>,

    /// Keyword used in default presets' display_name.
    /// Used to deduce IDs of default presets for config groups so they can be used in ResetToDefault() methods.
    /// If won't find default preset or any preset at that will use 0 as the preset ID.
    /// Case insensitive.
    #[clap(long, default_value="default", display_order=5)]
    pub default_preset_keyword: String
}