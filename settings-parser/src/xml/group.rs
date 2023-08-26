use roxmltree::Node;

use crate::utils::{validate_name, node_pos};

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
}


impl TryFrom<&Node<'_, '_>> for Group {
    type Error = String;

    fn try_from(node: &Node) -> Result<Self, Self::Error> {
        let tag_name = node.tag_name().name();
        if tag_name != "Group" {
            return Err(format!("Wrong XML node. Expected Group, received {}", tag_name))
        }


        let group_id = node.attribute("id");
        if group_id.is_none() {
            return Err(format!("No id attribute found for Group tag at {}", node_pos(node)));   
        }
        let group_id = group_id.unwrap().to_owned();

        if let Err(err) = validate_name(&group_id) {
            return Err(format!("Invalid Group id {} at {}: {}", group_id, node_pos(node), err));
        }


        let group_display_name = node.attribute("displayName");
        if group_display_name.is_none() {
            return Err(format!("No displayName attribute found for Group tag at {}", node_pos(node)));   
        }
        let group_display_name = group_display_name.unwrap().to_owned();

        // let class_name = node.attribute("msfClass").map(|s| s.to_string());

        let variable_name = node.attribute("msfVariable").map(|s| s.to_string());

        //TODO write utility functions (which include validation) for parsing data types
        let ignore;
        if let Some(val) = node.attribute("msfIgnore") {
            match val {
                "true" => ignore = Some(true),
                "false" => ignore = Some(false),
                _ => {
                    return Err(format!("Invalid value for attribute msfIgnore at {}", node_pos(node)));
                }
            }
        } else {
            ignore = None;
        }

        let mut preset_elements = Vec::<(usize, &str)>::new();
        if let Some(presets_array_node) = node.children().find(|n| n.has_tag_name("PresetsArray")) {
            let preset_nodes = presets_array_node.children().filter(|n| n.has_tag_name("Preset"));

            if preset_nodes.clone().count() == 0 {
                return Err(format!("No Preset nodes found in PresetsArray at {}", node_pos(&presets_array_node)));
            }

            for preset_node in preset_nodes {
                let id = preset_node.attribute("id");
                if id.is_none() {
                    return Err(format!("No id attribute found for Preset node at {}", node_pos(node)))
                }

                let id = str::parse::<usize>(id.unwrap());
                if let Err(_) = id {
                    return Err(format!("Invalid id attribute at {}: expected a number", node_pos(&preset_node)));
                }

                let id = id.unwrap();


                let display_name = preset_node.attribute("displayName");
                if display_name.is_none() {
                    return Err(format!("No displayName attribute found for Preset node at {}", node_pos(node)))
                }

                let display_name = display_name.unwrap();


                preset_elements.push((id, display_name))
            }
        }

        preset_elements.sort_by(|(i1, _), (i2, _)| i1.cmp(i2));

        let presets_array = preset_elements.iter()
                            .map(|&(_, dn)| dn.to_owned())
                            .collect();


        let default_preset_index = node.children()
                                   .find(|n| n.has_tag_name("PresetsArray"))
                                   .and_then(|n| n.attribute("msfDefault"))
                                   .map(|s| str::parse::<u8>(s));
                                
        if default_preset_index.is_some() {
            if let Err(err) = default_preset_index.as_ref().unwrap() {
                return Err(format!("msfDefault attribute at {} contains invalid value: {}", node_pos(node), err));
            }
        }

        let default_preset_index = default_preset_index.map(|r| r.unwrap());

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
        })
    }
}
