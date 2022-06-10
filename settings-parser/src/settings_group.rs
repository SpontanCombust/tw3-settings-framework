use crate::{settings_var::SettingsVar, to_witcher_script::ToWitcherScript};

#[derive(Default)]
pub struct SettingsGroup {
    pub master_name: String,
    pub id: String,
    pub name: String,
    pub default_preset_index: Option<u8>,
    pub vars: Vec<SettingsVar>
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