use roxmltree::Node;

use crate::utils::{
    parse_attribute_bool, 
    parse_attribute_string, 
    parse_attribute_number_required, 
    parse_attribute_string_required,
    node_pos
};



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

        let is_enum = parse_attribute_bool(node, "msfIsEnum")?;
        let enum_type = parse_attribute_string(node, "msfEnum", true)?;

        let option_nodes = node.children()
                               .filter(|n| n.has_tag_name("Option"))
                               .collect::<Vec<_>>();

        if option_nodes.is_empty() {
            return Err(format!("OptionsArray node at {} is missing Option nodes", node_pos(node)));
        }

        let mut options_indexed = Vec::<(usize, OptionsArrayOption)>::new(); 
        for option_node in option_nodes {
            let id = parse_attribute_number_required::<usize>(&option_node, "id")?;
            let display_name = parse_attribute_string_required(&option_node, "displayName", true)?;
            let enum_value_suffix = parse_attribute_string(&option_node, "msfEnumValue", true)?;
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