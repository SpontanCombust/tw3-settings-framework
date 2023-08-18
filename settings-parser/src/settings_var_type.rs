use crate::{
    xml::{display_type::DisplayType, var::Var}, 
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
    pub fn from(var: &Var, master_class_name: &str, prefixes: &Vec<String>) -> Result<Option<Self>, String> {
        match &var.display_type {
            DisplayType::Toggle => {
                Ok(Some(SettingsVarType::Bool))
            },
            DisplayType::Slider { min, max, div } => {
                if is_integral_range(*min, *max, *div) {
                    Ok(Some(SettingsVarType::Int {
                        min: *min, 
                        max: *max
                    }))
                } else {
                    Ok(Some(SettingsVarType::Float { 
                        min: *min as f32,
                        max: *max as f32 
                    }))
                }
            },
            DisplayType::Options(options_array) => {
                if options_array.is_enum.unwrap_or(true) {
                    let settings_enum = SettingsEnum::from(options_array, &var.id, master_class_name, prefixes)?;
                    Ok(Some(SettingsVarType::Enum {
                        val: settings_enum,
                        val_mapping: None
                    }))
                } else {
                    Ok(Some(SettingsVarType::Int { 
                        min: 0, 
                        max: (options_array.options.len() - 1) as i32
                    }))
                }
            },
            DisplayType::SubtleSeparator => {
                Ok(None)
            },
        }
    }
}