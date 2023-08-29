use roxmltree::Node;

use crate::utils::{
    parse_attribute_string_required, 
    parse_attribute_string, 
    parse_attribute_bool
};

use super::display_type::DisplayType;



pub struct Var {
    pub id: String,
    pub variable_name: Option<String>,
    pub display_name: String,
    pub display_type: DisplayType,
    pub ignore: Option<bool>,
}


impl TryFrom<&Node<'_, '_>> for Var {
    type Error = String;

    fn try_from(node: &Node) -> Result<Self, Self::Error> {
        let tag_name = node.tag_name().name();
        if tag_name != "Var" {
            return Err(format!("Wrong XML node. Expected Var, received {}", tag_name))
        }

        let id = parse_attribute_string_required(node, "id", true)?;
        let display_name = parse_attribute_string_required(node, "displayName", false)?;
        let variable_name = parse_attribute_string(node, "msfVariable", true)?;
        let ignore = parse_attribute_bool(node, "msfIgnore")?;
        
        Ok(Var {
            id: id.to_owned(),
            variable_name,
            display_name: display_name.to_owned(),
            display_type: DisplayType::try_from(node)?,
            ignore
        })
    }
}