use roxmltree::{Document, Node};

use crate::utils::{
    parse_attribute_string, 
    parse_attribute_string_required, parse_attribute_bool
};

use super::group::Group;



pub struct UserConfig {
    pub class_name: String,
    pub mod_version: Option<String>,
    pub mod_prefixes: Vec<String>,
    pub groups: Vec<Group>,
    pub validate: Option<bool>
}


impl TryFrom<&Document<'_>> for UserConfig {
    type Error = String;

    fn try_from(doc: &Document) -> Result<Self, Self::Error> {
        if let Some(root_node) = doc.descendants().find(|n| n.has_tag_name("UserConfig")) {
            let class_name = parse_attribute_string_required(&root_node, "msfClass", true)?;
            let mod_version = parse_attribute_string(&root_node, "msfVersion", false)?;

            let mod_prefixes = if let Some(prefixes) = root_node.attribute("msfPrefix") {
                prefixes.split(';')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
            } else {
                Vec::new()
            };

            let validate = parse_attribute_bool(&root_node, "msfValidate")?;

            
            let group_nodes: Vec<Node> = root_node.children()
                                        .filter(|n| n.has_tag_name("Group"))
                                        .collect();

            if group_nodes.is_empty() {
                return Err("No Groups found inside UserConfig".to_string());
            }

            let mut groups = Vec::new();
            for group_node in group_nodes {
                let group = Group::try_from(&group_node)?;
                if !group.visible_vars.is_empty() {
                    groups.push(group);
                } else {
                    println!("Group {} has no vars and will be ignored.", group.id);
                }
            }

            Ok(UserConfig { 
                class_name,
                mod_version,
                mod_prefixes,
                groups,
                validate
            })
        } else {
            Err("No UserConfig node found in the document".into())
        }
    }
}