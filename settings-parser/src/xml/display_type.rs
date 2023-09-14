use roxmltree::Node;

use crate::utils::{
    parse_attribute_string_required, 
    node_pos
};

use super::options_array::OptionsArray;


pub enum DisplayType {
    Toggle,
    Options(OptionsArray), // array of displayNames
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

        let display_type = parse_attribute_string_required(node, "displayType", false)?;

        if display_type == "TOGGLE" {
            Ok(DisplayType::Toggle)
        } else if display_type == "OPTIONS" {
            if let Some(options_array_node) = node.children().find(|ch| ch.has_tag_name("OptionsArray")) {
                let options_array = OptionsArray::try_from(&options_array_node)?;
                Ok(DisplayType::Options(options_array))
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