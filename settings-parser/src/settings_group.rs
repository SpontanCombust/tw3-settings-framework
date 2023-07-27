use crate::{settings_var::SettingsVar, traits::{ToWitcherScriptType, WitcherScript}, cli::CLI, xml::group::Group, utils::id_to_script_name};

pub struct SettingsGroup {
    pub id: String, // id attribute in the Var node
    pub class_name: String, // name of the class for this group in WitcherScript
    pub var_name: String, // name of an instance of the class for this group in WitcherScript
    pub default_preset_index: u8,
    pub vars: Vec<SettingsVar>
}

impl SettingsGroup {
    pub fn from(xml_group: &Group, cli: &CLI) -> Self {    
        let default_preset_index = xml_group.presets_array.iter().enumerate()
                                            .find(|(_, preset)| preset.contains(&cli.default_preset_keyword.to_lowercase()))
                                            .map(|(i, _)| i as u8)
                                            .unwrap_or(0);
            
        let id = xml_group.id.clone();
        let var_name = id_to_script_name(&id, &cli.omit_prefix);
        let class_name = format!("{}_{}", cli.settings_master_name, var_name); //TODO styling modificator
        let mut setting_vars = Vec::<SettingsVar>::new();

        for xml_var in &xml_group.visible_vars {
            if let Some(setting_var) = SettingsVar::from(xml_var, cli) {
                setting_vars.push(setting_var);
            }
        }

        SettingsGroup {
            id,
            class_name,
            var_name,
            default_preset_index,
            vars: setting_vars,
        }
    }
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
          .push_line(&format!("default {} = {};", SETTINGS_GROUP_DEFAULT_PRESET_VAR_NAME, group.default_preset_index));
}