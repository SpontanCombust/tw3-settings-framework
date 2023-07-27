use roxmltree::Node;

use crate::utils::{node_pos, validate_name};

pub enum DisplayType {
    Toggle,
    Options(Vec<String>), // array of displayNames
    Slider {
        min: i32,
        max: i32,
        div: i32
    },
    SubtleSeparator
}


impl TryFrom<&Node<'_, '_>> for DisplayType {
    type Error = String;

    fn try_from(node: &Node) -> Result<Self, Self::Error> {
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
            Ok(DisplayType::Toggle)
        } else if display_type == "OPTIONS" {
            if let Some(options_array_node) = node.children().find(|ch| ch.has_tag_name("OptionsArray")) {
                let option_nodes = options_array_node.children()
                                   .filter(|n| n.has_tag_name("Option"))
                                   .collect::<Vec<_>>();

                if option_nodes.is_empty() {
                    return Err(format!("OptionsArray node at {} is missing Option nodes", node_pos(&options_array_node)));
                }


                let mut option_elements = Vec::<(usize, &str)>::new();
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

                    let display_name = display_name.unwrap();


                    option_elements.push((id, display_name));
                }

                option_elements.sort_by(|(i1, _), (i2, _)| i1.cmp(i2));
                
                let display_names = option_elements.iter()
                                    .map(|&(_, dn)| dn.to_owned())
                                    .collect();

                Ok(DisplayType::Options(display_names))
            } else {
                Err(format!("No OptionsArray node found in var with OPTIONS displayType at {}", node_pos(node)))
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
                    return Err(format!("Slider min value is greater or equal to max value: {}", min));
                }

                Ok(DisplayType::Slider { 
                    min, 
                    max, 
                    div
                })
            }
        } else if display_type == "SUBTLE_SEPARATOR" {
            Ok(DisplayType::SubtleSeparator)
        } else {
            Err(format!("Unsupported display type: {}", display_type))
        }
    }
}