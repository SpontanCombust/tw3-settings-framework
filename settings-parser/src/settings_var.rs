use roxmltree::Node;

use crate::{var_type::VarType, traits::{ToWitcherScriptType, FromXmlNode, WitcherScript}, cli::CLI, utils::{node_pos, validate_name, id_to_script_name, is_integral_range}};

pub struct SettingsVar {
    pub id: String, // id attribute in the Var node
    pub var_name: String, // name of a variable inside a group class in WitcherScript
    pub var_type: VarType
}

impl FromXmlNode for SettingsVar {
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



impl ToWitcherScriptType for SettingsVar {
    fn ws_type_name(&self) -> String {
        match &self.var_type {
            VarType::Toggle => String::from("bool"),
            VarType::Options {enum_name, ..} => enum_name.as_deref().unwrap_or("int").into(),
            VarType::Slider {min, max, div} => {
                if is_integral_range(*min, *max, *div) {
                    String::from("int")
                } else {
                    String::from("float")
                }
            }
        }
    }

    fn ws_type_definition(&self, buffer: &mut WitcherScript) -> bool {
        match &self.var_type {
            VarType::Options {options_array, enum_name} => {
                if let Some(enum_name) = &enum_name {
                    buffer.push_line(&format!("enum {}", enum_name))
                          .push_indent("{");
                    
                    for i in 0..options_array.len() {
                        buffer.push_line(&format!("{} = {},", options_array[i], i));
                    }

                    buffer.pop_indent("}");
                    true
                } else {
                    false
                }
            },
            _ => false
        }
    }
}