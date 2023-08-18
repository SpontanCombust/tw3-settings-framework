use roxmltree::Node;

use crate::utils::{node_pos, validate_name};

pub struct OptionsArray {
    pub is_enum: Option<bool>,
    pub enum_type: Option<String>,
    pub options: Vec<OptionsArrayOption>
}

pub struct OptionsArrayOption {
    pub display_name: String,
    pub enum_value_suffix: Option<String>
}


impl TryFrom<&Node<'_, '_>> for OptionsArray {
    type Error = String;

    fn try_from(node: &Node) -> Result<Self, Self::Error> {
        let tag_name = node.tag_name().name();
        if tag_name != "OptionsArray" {
            return Err(format!("Wrong XML node. Expected OptionsArray, received {}", tag_name))
        }

        let is_enum;
        if let Some(val) = node.attribute("msfIsEnum") {
            match val {
                "true" => is_enum = Some(true),
                "false" => is_enum = Some(false),
                _ => {
                    return Err(format!("Invalid value for attribute msfIsEnum at {}", node_pos(node)));
                }
            }
        } else {
            is_enum = None;
        }

        let enum_type = node.attribute("msfEnum").map(|s| s.to_string());


        let option_nodes = node.children()
                               .filter(|n| n.has_tag_name("Option"))
                               .collect::<Vec<_>>();

        if option_nodes.is_empty() {
            return Err(format!("OptionsArray node at {} is missing Option nodes", node_pos(node)));
        }

        let mut options_indexed = Vec::<(usize, OptionsArrayOption)>::new(); 
        for option_node in option_nodes {
            let id = option_node.attribute("id");
            if id.is_none() {
                return Err(format!("Option node at {} is missing id attribute", node_pos(&option_node)));
            }

            let id = str::parse::<usize>(id.unwrap());
            if let Err(_) = id {
                return Err(format!("Invalid id attribute at {}: expected a number", node_pos(&option_node)));
            }
            let id = id.unwrap();
            
            let display_name = option_node.attribute("displayName");
            if display_name.is_none() {
                return Err(format!("Option node at {} is missing displayName attribute", node_pos(&option_node)));
            } else if let Err(err) = validate_name(display_name.unwrap()) {
                return Err(format!("Invalid displayName attribute at {}: {}", node_pos(&option_node), err));
            }
            let display_name = display_name.unwrap().to_string();

            let enum_value_suffix = option_node.attribute("msfEnumValue").map(|s| s.to_string());

            options_indexed.push((id, OptionsArrayOption { display_name, enum_value_suffix }));
        }

        options_indexed.sort_by(|(i1, _), (i2, _)| i1.cmp(i2));

        let options = options_indexed.into_iter()
                      .map(|(_, option)| option)
                      .collect();

        Ok(OptionsArray { 
            is_enum, 
            enum_type, 
            options 
        })
    }
}