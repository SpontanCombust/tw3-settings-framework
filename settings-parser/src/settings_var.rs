use roxmltree::Node;

use crate::{var_type::VarType, traits::{ToWitcherScript, FromXMLNode}, cli::CLI, utils::{node_pos, validate_name, id_to_script_name, is_integral_range}};

pub struct SettingsVar {
    pub id: String,
    pub var_name: String,
    pub var_type: VarType
}

impl FromXMLNode for SettingsVar {
    fn from_xml_node(node: &Node, cli: &CLI) -> Result<Option<Self>, String> {
        let tag_name = node.tag_name().name();
        if tag_name != "Var" {
            return Err(format!("Wrong XML node. Expected Var, received {}", tag_name))
        }

        let var_id = match node.attribute("id") {
            Some(id) => id,
            None => {
                return Err(format!("Var node without id found at {}", node_pos(node)));
            }
        };
    
        if let Err(err) = validate_name(var_id) {
            return Err(format!("Invalid Var id {} at {}: {}", var_id, node_pos(node), err));
        }
        
        if let Some(var_type) = VarType::from_xml_node(node, cli)? {
            Ok(Some(SettingsVar {
                id: var_id.to_owned(),
                var_name: id_to_script_name(var_id, &cli.omit_prefix),
                var_type
            }))
        } else {
            Ok(None)
        }
    }
}



impl ToWitcherScript for SettingsVar {
    fn ws_type_name(&self) -> String {
        match &self.var_type {
            VarType::Toggle => String::from("bool"),
            VarType::Options {enum_type, ..} => {
                if let Some(enum_type) = &enum_type {
                    enum_type.to_owned()
                } else {
                    String::from("int")
                }
            },
            VarType::Slider {min, max, div} => {
                if is_integral_range(*min, *max, *div) {
                    String::from("int")
                } else {
                    String::from("float")
                }
            }
        }
    }

    fn ws_type_definition(&self, buffer: &mut String) -> bool {
        match &self.var_type {
            VarType::Options {options_array, enum_type} => {
                if let Some(enum_type) = &enum_type {
                    buffer.push_str(&format!("enum {}\n", enum_type));
                    buffer.push_str("{\n");
                    
                    for i in 0..options_array.len() {
                        buffer.push_str(&format!("\t{} = {},\n", options_array[i], i));
                    }

                    buffer.push_str("}\n");
                    true
                } else {
                    false
                }
            },
            _ => false
        }
    }
}