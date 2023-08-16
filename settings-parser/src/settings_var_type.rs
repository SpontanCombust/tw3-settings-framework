use crate::{
    xml::{display_type::DisplayType, var::Var}, 
    cli::{CLI, OptionParsingMode}, 
    utils::is_integral_range, 
    settings_enum::{SettingsEnum, SettingsEnumValueMapping}
};

pub enum SettingsVarType {
    Bool,
    Int {
        min: i32,
        max: i32
    },
    Float {
        min: f32,
        max: f32
    },
    Enum {
        val: SettingsEnum,
        val_mapping: Option<SettingsEnumValueMapping>
    }
}

impl SettingsVarType {
    pub fn from(var: &Var, master_class_name: &str, cli: &CLI) -> Option<Self> {
        match &var.display_type {
            DisplayType::Toggle => {
                Some(SettingsVarType::Bool)
            },
            DisplayType::Slider { min, max, div } => {
                if is_integral_range(*min, *max, *div) {
                    Some(SettingsVarType::Int {
                        min: *min, 
                        max: *max
                    })
                } else {
                    Some(SettingsVarType::Float { 
                        min: *min as f32,
                        max: *max as f32 
                    })
                }
            },
            DisplayType::Options(options_array) => {
                match cli.option_parsing_mode {
                    OptionParsingMode::Ints => {
                        Some(SettingsVarType::Int { 
                            min: 0, 
                            max: (options_array.len() - 1) as i32
                        })
                    },
                    OptionParsingMode::Enums | OptionParsingMode::EnumsStrict => {
                        Some(SettingsVarType::Enum {
                            val: SettingsEnum::from(options_array, &var.id, master_class_name, cli),
                            val_mapping: None
                        })
                    }
                }
            },
            DisplayType::SubtleSeparator => {
                None
            },
        }
    }
}