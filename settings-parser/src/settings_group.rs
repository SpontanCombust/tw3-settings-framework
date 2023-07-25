use roxmltree::Node;

use crate::{settings_var::SettingsVar, traits::{ToWitcherScriptType, FromXmlNode, WitcherScript}, cli::CLI, utils::{validate_name, node_pos, id_to_script_name}};

pub struct SettingsGroup {
    pub id: String, // id attribute in the Var node
    pub class_name: String, // name of the class for this group in WitcherScript
    pub var_name: String, // name of an instance of the class for this group in WitcherScript
    pub default_preset_index: Option<u8>,
    pub vars: Vec<SettingsVar>
}

impl FromXmlNode for SettingsGroup {
    fn from_xml_node(node: &Node, cli: &CLI) -> Result<Option<Self>, String> {
        let tag_name = node.tag_name().name();
        if tag_name != "Group" {
            return Err(format!("Wrong XML node. Expected Group, received {}", tag_name))
        }

        if let Some(group_id) = node.attribute("id") {
            if let Err(err) = validate_name(group_id) {
                return Err(format!("Invalid Group id {} at {}: {}", group_id, node_pos(node), err));
            }
    
            let mut default_preset_index: Option<u8> = None;
            if let Some(presets_array_node) = node.children().find(|n| n.has_tag_name("PresetsArray")) {
                default_preset_index = parse_presets_array_node(&presets_array_node, cli);
            }
    
            if let Some(visible_vars_node) = node.children().find(|n| n.has_tag_name("VisibleVars")) {
                let var_nodes: Vec<Node> = visible_vars_node.children().filter(|n| n.has_tag_name("Var")).collect();
    
                if var_nodes.is_empty() {
                    println!("Group {} at {} has no vars and will be ignored.", group_id, node_pos(node));
                    return Ok(None);
                }
    
                let id = group_id.to_owned();
                let var_name = id_to_script_name(group_id, &cli.omit_prefix);
                let class_name = format!("{}_{}", cli.settings_master_name, var_name); //TODO styling modificator
                let mut setting_vars = Vec::<SettingsVar>::new();
    
                for var_node in &var_nodes {
                    if let Some(settings_var) = SettingsVar::from_xml_node(&var_node, cli)? {
                        setting_vars.push(settings_var);
                    }
                }
    
                Ok(Some(SettingsGroup {
                    id,
                    class_name,
                    var_name,
                    default_preset_index,
                    vars: setting_vars,
                }))
            } else {
                println!("Group {} at {} has no vars and will be ignored.", group_id, node_pos(node));
                Ok(None)
            }
        } else {
            Err(format!("No id attribute found for Group tag at {}", node_pos(node)))
        }
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



const SETTINGS_GROUP_PARENT_CLASS: &str = "ISettingsGroup";
const SETTINGS_GROUP_ID_VAR_NAME: &str = "id";
const SETTINGS_GROUP_DEFAULT_PRESET_VAR_NAME: &str = "defaultPresetIndex";

impl ToWitcherScriptType for SettingsGroup {
    fn ws_type_name(&self) -> String {
        self.class_name.clone()
    }

    fn ws_type_definition(&self, buffer: &mut WitcherScript) -> bool {
        buffer.push_line(&format!("class {} extends {}", self.ws_type_name(), SETTINGS_GROUP_PARENT_CLASS));
        buffer.push_indent("{");

        group_class_variables(self, buffer);
        
        buffer.new_line();
        group_default_variable_values(self, buffer);

        buffer.pop_indent("}");

        true
    }
}

fn group_class_variables(group: &SettingsGroup, buffer: &mut WitcherScript) {
    for var in &group.vars {
        buffer.push_line(&format!("public var {} : {};", var.var_name, var.ws_type_name()));
    }
}

fn group_default_variable_values(group: &SettingsGroup, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("default {} = '{}';", SETTINGS_GROUP_ID_VAR_NAME, group.id))
          .push_line(&format!("default {} = {};", SETTINGS_GROUP_DEFAULT_PRESET_VAR_NAME, group.default_preset_index.unwrap_or(0)));
}