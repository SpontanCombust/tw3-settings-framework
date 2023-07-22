use roxmltree::Node;

use crate::utils::node_pos;

#[derive(Debug)]
pub enum VarType {
    Toggle,
    Options(OptionsVarType),
    Slider(SliderVarType)
    // SubtleSeparator not included as it's just a cosmetic var
}


#[derive(Debug)]
pub struct OptionsVarType {
    pub options_array: Vec<String>
}


#[derive(Debug)]
pub struct SliderVarType {
    pub min: i32,
    pub max: i32,
    pub div: i32
}

impl SliderVarType {
    pub fn is_integral(&self) -> bool {
        (self.max - self.min) % self.div == 0
    }
}


impl VarType {
    pub fn from_xml(var_node: &Node) -> Result<Option<VarType>, String> {
        let display_type = match var_node.attribute("displayType") {
            Some(dt) => dt,
            None => {
                println!("Var node without displayType attribute found in at {}", node_pos(var_node));
                return Ok(None);
            }
        };

        if display_type == "TOGGLE" {
            return Ok(Some(VarType::Toggle));
        } else if display_type == "OPTIONS" {
            if let Some(options_array_node) = var_node.children().find(|ch| ch.has_tag_name("OptionsArray")) {
                let option_nodes = options_array_node.children()
                    .filter(|n| n.has_tag_name("Option"))
                    .collect::<Vec<_>>();

                if option_nodes.is_empty() {
                    return Err(format!("OptionsArray node at {} is missing Option nodes", node_pos(&options_array_node)));
                }

                let mut display_names = Vec::new();
                for option_node in option_nodes {
                    if let Some(display_name) = option_node.attribute("displayName") {
                        display_names.push(display_name.to_owned());
                    } else {
                        return Err(format!("Option node at {} is missing displayName attribute", node_pos(&option_node)));
                    }
                }

                Ok(Some(VarType::Options(OptionsVarType { options_array: display_names })))
            } else {
                return Err(format!("No OptionsArray node found in var with OPTIONS displayType at {}", node_pos(var_node)));
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

                Ok(Some(VarType::Slider(SliderVarType { min, max, div })))
            }
        } else if display_type == "SUBTLE_SEPARATOR" {
            Ok(None)
        } else {
            Err(format!("Unsupported display type: {}", display_type))
        }
    }
}