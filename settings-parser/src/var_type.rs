use crate::{xml::{display_type::DisplayType, var::Var}, cli::CLI, utils::{id_to_script_name, is_integral_range}};

//TODO rename to SettingsVarType
pub enum VarType {
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
        name: String,
        values: Vec<String>
    }
}

impl VarType {
    pub fn from(var: &Var, cli: &CLI) -> Option<Self> {
        match &var.display_type {
            DisplayType::Toggle => {
                Some(VarType::Bool)
            },
            DisplayType::Slider { min, max, div } => {
                if is_integral_range(*min, *max, *div) {
                    Some(VarType::Int {
                        min: *min, 
                        max: *max
                    })
                } else {
                    Some(VarType::Float { 
                        min: *min as f32,
                        max: *max as f32 
                    })
                }
            },
            DisplayType::Options(display_names) => {
                if cli.options_as_int {
                    None
                } else { 
                    let enum_name = format!("{}_{}", cli.settings_master_name, id_to_script_name(&var.id, &cli.omit_prefix));
                    let enum_values = display_names.iter()
                                        .map(|dn| format!("{}_{}", enum_name, id_to_script_name(&dn, &cli.omit_prefix)))
                                        .collect::<Vec<_>>();

                    Some(VarType::Enum { 
                        name: enum_name, 
                        values: enum_values 
                    })
                }
            },
            DisplayType::SubtleSeparator => {
                None
            },
        }
    }
}

pub fn common_str_prefix(v: &Vec<String>) -> Option<String> {
    // parsing should guarantee it won't be empty
    if v.len() <= 1 {
        return Some(v[0].to_owned());
    }

    let mut common_len = usize::MAX; 
    for i in 0..v.len() - 1 {
        let s1 = &v[i];
        let s2 = &v[i + 1];
        common_len = std::cmp::min(common_len, common_str_prefix_len(s1, s2));
    }

    if common_len == 0 {
        return None;
    }
    
    return Some(v[0][0..common_len].to_owned());
}

fn common_str_prefix_len(s1: &str, s2: &str) -> usize {
    let min_len = std::cmp::min(s1.len(), s2.len());

    for i in 0..min_len {
        if s1.chars().nth(i) != s2.chars().nth(i) {
            return i;
        }
    }

    min_len
}