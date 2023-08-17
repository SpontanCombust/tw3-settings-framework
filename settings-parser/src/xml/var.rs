use roxmltree::Node;

use crate::utils::{node_pos, validate_name};

use super::display_type::DisplayType;

pub struct Var {
    pub id: String,
    pub variable_name: Option<String>,
    pub display_name: String,
    pub display_type: DisplayType
}


impl TryFrom<&Node<'_, '_>> for Var {
    type Error = String;

    fn try_from(node: &Node) -> Result<Self, Self::Error> {
        let tag_name = node.tag_name().name();
        if tag_name != "Var" {
            return Err(format!("Wrong XML node. Expected Var, received {}", tag_name))
        }


        let id = match node.attribute("id") {
            Some(id) => id,
            None => {
                return Err(format!("Var node without id found at {}", node_pos(node)));
            }
        };
    
        if let Err(err) = validate_name(id) {
            return Err(format!("Invalid Var id {} at {}: {}", id, node_pos(node), err));
        }


        let display_name = match node.attribute("displayName") {
            Some(display_name) => display_name,
            None => {
                return Err(format!("Var node without displayName found at {}", node_pos(node)));
            },
        };

        let variable_name = node.attribute("msfVariable").map(|s| s.to_string());

        
        Ok(Var {
            id: id.to_owned(),
            variable_name,
            display_name: display_name.to_owned(),
            display_type: DisplayType::try_from(node)?,
        })
    }
}