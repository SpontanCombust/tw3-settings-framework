use clap::{Parser, ValueEnum};

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

    /// Prefix to omit from groups and vars when generating code. Case sensitive.
    /// Possible multiple cases.
    #[clap(long, short = 'p', display_order=1)]
    pub omit_prefix: Vec<String>,

    /// Controls how OPTION type vars are parsed into WitcherScript
    /// - ints:
    /// Treats options vars as regular ints instead of creating custom enum types for them.
    /// This essentially means the behaviour from before v0.5.
    /// - enums:
    /// Parses options vars into enums. Then tries to find vars that have the same set of displayName attributes in option node 
    /// and assigns them one common type.
    /// Requires that displayNames of all option nodes contain some prefix that determines their relation.
    /// If two option arrays contain the same set of possible values they are considered to have the same enum type.
    /// - enums-strict:
    /// Parses options vars into enums with an exception that having mutliple option arrays designated by the same prefix
    /// but having different sets of values is disallowed. This prevents possible user mistakes from happening. 
    #[clap(long, arg_enum, default_value="enums", verbatim_doc_comment, display_order=2)]
    pub option_parsing_mode: OptionParsingMode,

    /// Disables the generation of code for value correction.
    /// After reading from or before writing to user config values will no longer be checked if they adhere to the XML,
    /// e.g. if slider value is in a specified range.
    #[clap(long, display_order=3)]
    pub no_var_validation: bool,

    /// Prevents the settings object getter convenience function from being generated
    #[clap(long, display_order=4)]
    pub no_getter: bool
}

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OptionParsingMode {
    Ints,
    Enums,
    EnumsStrict,
}