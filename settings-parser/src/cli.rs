use clap::Parser;

#[derive(Parser)]
#[clap(name = "TW3 Settings Framework Parser")]
#[clap(version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown version"))]
#[clap(about = "Parses a mod menu XML file and outputs witcher script code representing settings of this menu", long_about=None)]
pub struct CLI {
    /// Path to the menu xml file. (REQUIRED)
    pub xml_file_path: String,

    /// Path of the WitcherScipt output file, by default it's made from the menu xml file name in the same directory.
    #[clap(long = "output", short = 'o', display_order=0)]
    pub output_ws_file_path: Option<String>,

    /// Causes the parser to throw an error if it will find occurances options that would generate unified enum type.
    /// More about unified enums in `doc/details.md`
    #[clap(long, display_order=1)]
    pub strict_enums: bool,

    /// Disables the generation of code for value correction.
    /// After reading from or before writing to user config values will no longer be checked if they adhere to the XML,
    /// e.g. if slider value is in a specified range.
    #[clap(long, display_order=2)]
    pub no_var_validation: bool,

    /// Prevents the settings object getter convenience function from being generated
    #[clap(long, display_order=3)]
    pub no_getter: bool
}
