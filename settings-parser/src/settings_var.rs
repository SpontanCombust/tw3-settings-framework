use roxmltree::Node;

use crate::{var_type::VarType, to_witcher_script::ToWitcherScript, cli::CLI, utils::{node_pos, validate_name, id_to_script_name}};

pub struct SettingsVar {
    pub id: String,
    pub name: String,
    pub var_type: VarType
}

impl SettingsVar {
    pub fn from_xml(var_node: &Node, group_id: &str, cli: &CLI) -> Result<Option<SettingsVar>, String> {
        let var_id = match var_node.attribute("id") {
            Some(id) => id,
            None => {
                println!("Var node without id found in Group {} at {}", group_id, node_pos(var_node));
                return Ok(None);
            }
        };
    
        if let Err(err) = validate_name(var_id) {
            return Err(format!("Invalid Var id {} at {}: {}", var_id, node_pos(var_node), err));
        }
    
        let var_display_type = match var_node.attribute("displayType") {
            Some(dt) => dt,
            None => {
                println!("Var node without displayType found in Group {} at {}", group_id, node_pos(var_node));
                return Ok(None);
            }
        };
    
        let var_type = match VarType::from_display_type(var_display_type) {
            Ok(vto) => match vto {
                Some(vt) => vt,
                None => return Ok(None),
            },
            Err(err) => {
                println!("Error parsing Var node's display_type in Group {} at {}: {}", group_id, node_pos(var_node), err);
                return Ok(None);
            }
        };
    
    
        return Ok(Some(SettingsVar {
            id: var_id.to_owned(),
            name: id_to_script_name(var_id, &cli.omit_prefix),
            var_type: var_type
        }));
    }
}



impl ToWitcherScript for SettingsVar {
    fn ws_type_name(&self) -> String {
        match &self.var_type {
            VarType::Toggle => String::from("bool"),
            VarType::Options => String::from("int"),
            VarType::SliderInt => String::from("int"),
            VarType::SliderFloat => String::from("float"),
        }
    }

    fn ws_code_body(&self) -> String {
        format!("var {} : {}", self.name, self.ws_type_name())
    }
}