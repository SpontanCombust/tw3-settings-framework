use roxmltree::{Document, Node};

use super::group::Group;

pub struct UserConfig {
    pub groups: Vec<Group>
}


impl TryFrom<&Document<'_>> for UserConfig {
    type Error = String;

    fn try_from(doc: &Document) -> Result<Self, Self::Error> {
        if let Some(root_node) = doc.descendants().find(|n| n.has_tag_name("UserConfig")) {
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
                groups 
            })
        } else {
            Err("No UserConfig node found in the document".into())
        }
    }
}