use roxmltree::Node;

use crate::{settings_var::SettingsVar, to_witcher_script::ToWitcherScript, cli::CLI, utils::{validate_name, node_pos, id_to_script_name}};

#[derive(Default)]
pub struct SettingsGroup {
    pub master_name: String,
    pub id: String,
    pub name: String,
    pub default_preset_index: Option<u8>,
    pub vars: Vec<SettingsVar>
}

impl SettingsGroup {
    pub fn from_xml(group_node: &Node, cli: &CLI) -> Result<Option<SettingsGroup>, String> {
        if let Some(group_id) = group_node.attribute("id") {
                    
            if let Err(err) = validate_name(group_id) {
                return Err(format!("Invalid Group id {} at {}: {}", group_id, node_pos(group_node), err));
            }
    
            let mut default_preset_index: Option<u8> = None;
            if let Some(presets_array_node) = group_node.children().find(|n| n.has_tag_name("PresetsArray")) {
                default_preset_index = SettingsGroup::parse_presets_array_node(&presets_array_node, cli);
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
                    match SettingsVar::from_xml(&var_node, group_id, cli) {
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
}




const SETTINGS_GROUP_PARENT_CLASS: &str = "ISettingsGroup";
const SETTINGS_GROUP_ID_VAR_NAME: &str = "id";
const SETTINGS_GROUP_DEFAULT_PRESET_VAR_NAME: &str = "defaultPresetIndex";

impl ToWitcherScript for SettingsGroup {
    fn ws_type_name(&self) -> String {
        format!("{}_{}", self.master_name, self.name)
    }

    fn ws_code_body(&self) -> String {
        let mut code = String::new();

        code += &format!("class {} extends {}\n", self.ws_type_name(), SETTINGS_GROUP_PARENT_CLASS);
        code += "{\n";

        code += &group_class_variables(self);

        code += "\n";
        code += &group_default_variable_values(self);

        code += "}\n";

        code
    }
}

fn group_class_variables(group: &SettingsGroup) -> String {
    let mut code = String::new();

    for var in &group.vars {
        code += &format!("\tpublic {};\n", var.ws_code_body());
    }

    return code;
}

fn group_default_variable_values(group: &SettingsGroup) -> String {
    let mut code = String::new();

    code += &format!("\tdefault {} = '{}';\n", SETTINGS_GROUP_ID_VAR_NAME, group.id);
    code += &format!("\tdefault {} = {};\n", SETTINGS_GROUP_DEFAULT_PRESET_VAR_NAME, group.default_preset_index.unwrap_or(0));

    return code;
}