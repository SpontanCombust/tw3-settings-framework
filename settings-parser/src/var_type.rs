use roxmltree::Node;

use crate::{utils::{node_pos, id_to_script_name, validate_name}, cli::CLI, traits::FromXMLNode};

#[derive(Debug)]
pub enum VarType {
    Toggle,
    Options {
        options_array: Vec<String>,
        enum_type: Option<String>
    },
    Slider {
        min: i32,
        max: i32,
        div: i32
    }
}


impl FromXMLNode for VarType {
    fn from_xml_node(node: &Node, cli: &CLI) -> Result<Option<Self>, String> {
        let tag_name = node.tag_name().name();
        if tag_name != "Var" {
            return Err(format!("Wrong XML node. Expected Var, received {}", tag_name))
        }

        let display_type = match node.attribute("displayType") {
            Some(dt) => dt,
            None => {
                return Err(format!("Var node without displayType attribute found at {}", node_pos(node)));
            }
        };

        if display_type == "TOGGLE" {
            return Ok(Some(VarType::Toggle));
        } else if display_type == "OPTIONS" {
            if let Some(options_array_node) = node.children().find(|ch| ch.has_tag_name("OptionsArray")) {
                let option_nodes = options_array_node.children()
                    .filter(|n| n.has_tag_name("Option"))
                    .collect::<Vec<_>>();

                if option_nodes.is_empty() {
                    return Err(format!("OptionsArray node at {} is missing Option nodes", node_pos(&options_array_node)));
                }

                let mut display_names = Vec::new();
                for option_node in option_nodes {
                    if let Some(display_name) = option_node.attribute("displayName") {
                        if let Err(err) = validate_name(display_name) {
                            return Err(format!("Invalid displayName attribute at {}: {}", node_pos(&option_node), err));
                        } else {
                            display_names.push(id_to_script_name(display_name, &cli.omit_prefix));
                        }
                    } else {
                        return Err(format!("Option node at {} is missing displayName attribute", node_pos(&option_node)));
                    }
                }

                let prefix = common_str_prefix(&display_names);
                let enum_type = if cli.options_as_int || prefix.is_none() { 
                    None 
                } else { 
                    Some(format!("{}_{}", cli.settings_master_name, common_str_prefix(&display_names).unwrap()))
                };

                let options_array = display_names.iter()
                                    .map(|dn| format!("{}_{}", cli.settings_master_name, dn))
                                    .collect::<Vec<_>>();

                Ok(Some(VarType::Options { 
                    options_array,
                    enum_type
                }))
            } else {
                return Err(format!("No OptionsArray node found in var with OPTIONS displayType at {}", node_pos(node)));
            }
        } 
        else if &display_type[0..6] == "SLIDER" {
            let spl: Vec<&str> = display_type.split(';').collect();

            if spl.len() == 1 {
                Err("No slider parameters given".to_string())
            } else if spl.len() != 4 {
                Err(format!("Invalid amount of slider parameters. Should be 3, is {}", spl.len() - 1))
            } else {
                let min = spl[1].parse::<i32>();
                if min.is_err() {
                    return Err(format!("Slider min value parse error: {}", min.unwrap_err()));   
                }

                let min = min.unwrap();


                let max = spl[2].parse::<i32>();
                if max.is_err() {
                    return Err(format!("Slider max value parse error: {}", max.unwrap_err()));   
                }

                let max = max.unwrap();


                let div = spl[3].parse::<i32>();
                if div.is_err() {
                    return Err(format!("Slider div value parse error: {}", div.unwrap_err()));   
                }

                let div = div.unwrap();
                if div <= 0 {
                    return Err("Slider div value must be greater than 0".to_string());
                }


                if min >= max {
                    return Err(format!("Slider min value is greater than max value: {}", min));
                }

                Ok(Some(VarType::Slider { 
                    min, 
                    max, 
                    div 
                }))
            }
        } else if display_type == "SUBTLE_SEPARATOR" {
            Ok(None)
        } else {
            Err(format!("Unsupported display type: {}", display_type))
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