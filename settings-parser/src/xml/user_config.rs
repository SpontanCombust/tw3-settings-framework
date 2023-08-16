use roxmltree::{Document, Node};

use super::group::Group;

pub struct UserConfig {
    pub class_name: String,
    pub mod_version: Option<String>,
    pub groups: Vec<Group>
}


impl TryFrom<&Document<'_>> for UserConfig {
    type Error = String;

    fn try_from(doc: &Document) -> Result<Self, Self::Error> {
        if let Some(root_node) = doc.descendants().find(|n| n.has_tag_name("UserConfig")) {
            let class_name = root_node.attribute("msfClass");
            if class_name.is_none() {
                return Err("No msfClass attribute found in UserConfig".to_string());
            }
            let class_name = class_name.unwrap().to_string();

            let mod_version = root_node.attribute("msfVersion").map(|s| s.to_string());

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
                groups 
            })
        } else {
            Err("No UserConfig node found in the document".into())
        }
    }
}