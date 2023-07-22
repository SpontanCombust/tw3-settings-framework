use roxmltree::Node;

use crate::{var_type::VarType, to_witcher_script::ToWitcherScript, cli::CLI, utils::{node_pos, validate_name, id_to_script_name}};

pub struct SettingsVar {
    pub id: String,
    pub name: String,
    pub var_type: VarType
}

impl SettingsVar {
    pub fn from_xml(var_node: &Node, group_id: &str, cli: &CLI) -> Result<Option<SettingsVar>, String> {
        //TODO rename with _node and _attr suffixes
        let var_id = match var_node.attribute("id") {
            Some(id) => id,
            None => {
                println!("Var node without id found in Group {} at {}", group_id, node_pos(var_node));
                return Ok(None);
            }
        };
    
        if let Err(err) = validate_name(var_id) {
            return Err(format!("Invalid Var id {} at {}: {}", var_id, node_pos(var_node), err));
        }
    
        let var_display_type = match var_node.attribute("displayType") {
            Some(dt) => dt,
            None => {
                println!("Var node without displayType found in Group {} at {}", group_id, node_pos(var_node));
                return Ok(None);
            }
        };
    
        let mut var_type = match VarType::from_display_type(var_display_type) {
            Ok(vto) => match vto {
                Some(vt) => vt,
                None => return Ok(None),
            },
            Err(err) => {
                println!("Error parsing Var node's display_type in Group {} at {}: {}", group_id, node_pos(var_node), err);
                return Ok(None);
            }
        };

        if let VarType::Options(options_var_type) = &mut var_type {
            if let Some(options_array) = var_node.children().find(|ch| ch.has_tag_name("OptionsArray")) {
                let option_nodes = options_array.children()
                    .filter(|n| n.has_tag_name("Option"))
                    .collect::<Vec<_>>();

                if option_nodes.is_empty() {
                    return Err(format!("OptionsArray node at {} is missing Option nodes", node_pos(&options_array)));
                }

                for option_node in option_nodes {
                    if let Some(display_name) = option_node.attribute("displayName") {
                        options_var_type.options_array.push(display_name.to_owned());
                    } else {
                        return Err(format!("Option node at {} is missing displayName attribute", node_pos(&option_node)));
                    }
                }
            } else {
                return Err(format!("No OptionsArray node found in var with OPTIONS displayType at {}", node_pos(var_node)));
            }                   
        }
    
        return Ok(Some(SettingsVar {
            id: var_id.to_owned(),
            name: id_to_script_name(var_id, &cli.omit_prefix),
            var_type: var_type
        }));
    }
}



impl ToWitcherScript for SettingsVar {
    fn ws_type_name(&self) -> String {
        match &self.var_type {
            VarType::Toggle => String::from("bool"),
            VarType::Options(_) => String::from("int"),
            VarType::Slider(slider) => {
                if slider.is_integral() {
                    String::from("int")
                } else {
                    String::from("float")
                }
            }
        }
    }

    fn ws_code_body(&self) -> String {
        format!("var {} : {}", self.name, self.ws_type_name())
    }
}