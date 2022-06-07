use crate::{settings_master::SettingsMaster, settings_group::SettingsGroup, settings_var::SettingsVar, var_type::VarType};
use roxmltree::{self, Document, Node};

pub fn parse_settings_xml(xml_text: String, settings_master_name: String, omit_prefix: Option<String>) -> Result<SettingsMaster, String> {
    validate_settings_master_name(&settings_master_name)?;

    let doc = match Document::parse(&xml_text) {
        Ok(doc) => doc,
        Err(err) => {
            return Err(err.to_string())
        }
    };
    
    let mut master = SettingsMaster::default();
    master.name = settings_master_name.clone();

    if let Some(root_node) = doc.descendants().find(|n| n.has_tag_name("UserConfig")) {
        let group_nodes: Vec<Node> = root_node.children().filter(|n| n.has_tag_name("Group")).collect();

        if group_nodes.is_empty() {
            println!("No Groups found inside UserConfig");
        }
        
        for group_node in &group_nodes {
            
            if let Some(group_id) = group_node.attribute("id") {

                if let Some(visible_vars_node) = group_node.children().find(|n| n.has_tag_name("VisibleVars")) {
                    let var_nodes: Vec<Node> = visible_vars_node.children().filter(|n| n.has_tag_name("Var")).collect();

                    if var_nodes.is_empty() {
                        println!("Group {} at {} has no vars and will be ignored.", group_id, node_pos(&group_node, &doc));
                        continue;
                    }

                    let mut sg = SettingsGroup::default();
                    sg.master_name = settings_master_name.clone();
                    sg.id = group_id.to_owned();
                    sg.name = id_to_name(group_id, &omit_prefix);

                    for var_node in &var_nodes {
                        let var_id = match var_node.attribute("id") {
                            Some(id) => id,
                            None => {
                                println!("Var node without id found in Group {} at {}", sg.id, node_pos(&var_node, &doc));
                                continue;
                            }
                        };
                 
                        let var_display_type = match var_node.attribute("displayType") {
                            Some(dt) => dt,
                            None => {
                                println!("Var node without displayType found in Group {} at {}", sg.id, node_pos(&var_node, &doc));
                                continue;
                            }
                        };

                        let var_type = match VarType::from_display_type(var_display_type) {
                            Ok(vt) => vt,
                            Err(err) => {
                                println!("Error parsing Var node in Group {} at {}: {}", sg.id, node_pos(&var_node, &doc), err);
                                continue;
                            }
                        };


                        sg.vars.push(SettingsVar {
                            id: var_id.to_owned(),
                            name: id_to_name(var_id, &omit_prefix),
                            var_type: var_type
                        });
                    }

                    master.groups.push(sg);
                }
                else {
                    println!("Group {} at {} has no vars and will be ignored.", group_id, node_pos(&group_node, &doc));
                }
            }
            else {
                println!("No id attribute found for Group tag at {}", node_pos(&group_node, &doc))
            }
        }
    }
    else {
        return Err("No UserConfig root node found".to_string());
    }

    return Ok(master);
}

fn validate_settings_master_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Settings master name cannot be empty".to_string());
    }
    if name.chars().nth(0).unwrap().is_numeric() {
        return Err("Settings master name cannot start with a number".to_string());
    }
    if name.chars().any(|c| !c.is_ascii_alphanumeric() && c != '_') {
        return Err("Settings master name must be alphanumeric with no spaces".to_string());
    }

    return Ok(());
}

fn node_pos(node: &Node, doc: &Document) -> String {
    let pos = doc.text_pos_at(node.range().start);
    format!("line {}, column {}", pos.row, pos.col)
}

fn id_to_name(id: &str, omit_prefix: &Option<String>) -> String {
    if let Some(prefix) = omit_prefix {
        if id.starts_with(prefix) {
            return id[prefix.len()..].to_string();
        }
    } 

    return id.to_string();
}