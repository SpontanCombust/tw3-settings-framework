use crate::{settings_master::SettingsMaster, settings_group::SettingsGroup, settings_var::SettingsVar, var_type::VarType, CLI};
use roxmltree::{self, Document, Node};

pub fn parse_settings_xml(xml_text: String, cli: &CLI) -> Result<SettingsMaster, String> {
    if let Err(err) = validate_name(&cli.settings_master_name) {
        return Err(format!("Invalid settings master name: {}", err));
    }

    let doc = match Document::parse(&xml_text) {
        Ok(doc) => doc,
        Err(err) => {
            return Err(err.to_string())
        }
    };
    
    let mut master = SettingsMaster::default();
    master.name = cli.settings_master_name.clone();
    master.mod_version = cli.mod_version.clone();

    if let Some(root_node) = doc.descendants().find(|n| n.has_tag_name("UserConfig")) {
        let group_nodes: Vec<Node> = root_node.children().filter(|n| n.has_tag_name("Group")).collect();

        if group_nodes.is_empty() {
            return Err("No Groups found inside UserConfig".to_string());
        }
        
        for group_node in &group_nodes {
            match parse_group_node(group_node, cli) {
                Ok(group_opt) => {
                    if let Some(group) = group_opt {
                        master.groups.push(group);
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }
    else {
        return Err("No UserConfig root node found".to_string());
    }

    return Ok(master);
}

fn validate_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("name cannot be empty".to_string());
    }
    if name.chars().nth(0).unwrap().is_numeric() {
        return Err("name cannot start with a number".to_string());
    }
    if name.chars().any(|c| !c.is_ascii_alphanumeric() && c != '_') {
        return Err("name can only have alphanumeric characters and underscores and have no spaces".to_string());
    }

    return Ok(());
}

fn parse_group_node(group_node: &Node, cli: &CLI) -> Result<Option<SettingsGroup>, String> {
    if let Some(group_id) = group_node.attribute("id") {
                
        if let Err(err) = validate_name(group_id) {
            return Err(format!("Invalid Group id {} at {}: {}", group_id, node_pos(group_node), err));
        }

        let mut default_preset_index: Option<u8> = None;
        if let Some(presets_array_node) = group_node.children().find(|n| n.has_tag_name("PresetsArray")) {
            default_preset_index = parse_presets_array_node(&presets_array_node, cli);
        }

        if let Some(visible_vars_node) = group_node.children().find(|n| n.has_tag_name("VisibleVars")) {
            let var_nodes: Vec<Node> = visible_vars_node.children().filter(|n| n.has_tag_name("Var")).collect();

            if var_nodes.is_empty() {
                println!("Group {} at {} has no vars and will be ignored.", group_id, node_pos(group_node));
                return Ok(None);
            }

            let mut sg = SettingsGroup::default();
            sg.master_name = cli.settings_master_name.to_owned();
            sg.id = group_id.to_owned();
            sg.name = id_to_script_name(group_id, &cli.omit_prefix);
            sg.default_preset_index = default_preset_index;

            for var_node in &var_nodes {
                match parse_var_node(&var_node, group_id, cli) {
                    Ok(var_opt) => {
                        if let Some(var) = var_opt {
                            sg.vars.push(var);
                        }
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }

            return Ok(Some(sg));
        }
        else {
            println!("Group {} at {} has no vars and will be ignored.", group_id, node_pos(group_node));
            return Ok(None);
        }
    }
    else {
        println!("No id attribute found for Group tag at {}", node_pos(group_node));
        return Ok(None);
    }
}

fn parse_presets_array_node(presets_array_node: &Node, cli: &CLI) -> Option<u8> {
    for preset_node in presets_array_node.children() {
        if preset_node.has_tag_name("Preset") && preset_node.has_attribute("id") && preset_node.has_attribute("displayName") {
            if preset_node.attribute("displayName").unwrap().contains(&cli.default_preset_keyword.to_lowercase()) {
                return preset_node.attribute("id").unwrap().parse::<u8>().ok();
            }
        }
    }

    return None;
}

fn parse_var_node(var_node: &Node, group_id: &str, cli: &CLI) -> Result<Option<SettingsVar>, String> {
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

fn node_pos(node: &Node) -> String {
    let pos = node.document().text_pos_at(node.range().start);
    format!("line {}, column {}", pos.row, pos.col)
}

fn id_to_script_name(id: &str, omit_prefix: &Option<String>) -> String {
    if let Some(prefix) = omit_prefix {
        if id.starts_with(prefix) {
            return id[prefix.len()..].to_string();
        }
    } 

    return id.to_string();
}