use roxmltree::Node;

use crate::utils::{
    parse_attribute_string_required, 
    parse_attribute_string, 
    parse_attribute_bool, 
    parse_attribute_number_required, 
    parse_attribute_number, 
    node_pos
};

use super::var::Var;



pub struct Group {
    pub id: String,
    pub display_name: String,
    // pub class_name: Option<String>,
    pub variable_name: Option<String>,
    pub presets_array: Vec<String>,
    pub default_preset_index: Option<u8>,
    pub visible_vars: Vec<Var>,
    pub ignore: Option<bool>,
    pub validate: Option<bool>
}


impl TryFrom<&Node<'_, '_>> for Group {
    type Error = String;

    fn try_from(node: &Node) -> Result<Self, Self::Error> {
        let tag_name = node.tag_name().name();
        if tag_name != "Group" {
            return Err(format!("Wrong XML node. Expected Group, received {}", tag_name))
        }

        let group_id = parse_attribute_string_required(node, "id", true)?;
        let group_display_name = parse_attribute_string_required(node, "displayName", false)?;
        // let class_name = parse_attribute_string(node, "msfClass", true)?;
        let variable_name = parse_attribute_string(node, "msfVariable", true)?;
        let ignore = parse_attribute_bool(node, "msfIgnore")?;
        let validate = parse_attribute_bool(node, "msfValidate")?;

        let mut preset_elements = Vec::<(usize, String)>::new();
        let mut default_preset_index = None;
        if let Some(presets_array_node) = node.children().find(|n| n.has_tag_name("PresetsArray")) {
            let preset_nodes = presets_array_node.children().filter(|n| n.has_tag_name("Preset"));

            if preset_nodes.clone().count() == 0 {
                return Err(format!("No Preset nodes found in PresetsArray at {}", node_pos(&presets_array_node)));
            }

            for preset_node in preset_nodes {
                let id = parse_attribute_number_required::<usize>(&preset_node, "id")?;
                let display_name = parse_attribute_string_required(&preset_node, "displayName", false)?;
                preset_elements.push((id, display_name))
            }

            default_preset_index = parse_attribute_number::<u8>(&presets_array_node, "msfDefault")?
        }

        preset_elements.sort_by(|(i1, _), (i2, _)| i1.cmp(i2));

        let presets_array = preset_elements.into_iter()
                            .map(|(_, dn)| dn)
                            .collect();


        let mut visible_vars = Vec::new();
        if let Some(visible_vars_node) = node.children().find(|n| n.has_tag_name("VisibleVars")) {
            let var_nodes = visible_vars_node.children().filter(|n| n.has_tag_name("Var"));
            for var_node in var_nodes {
                visible_vars.push(Var::try_from(&var_node)?);
            }
        }

        Ok(Group {
            id: group_id,
            display_name: group_display_name,
            // class_name,
            variable_name,
            ignore,
            presets_array,
            default_preset_index,
            visible_vars,
            validate
        })
    }
}
