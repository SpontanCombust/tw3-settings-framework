use crate::settings_group::SettingsGroup;

#[derive(Default)]
pub struct SettingsMaster {
    pub name: String,
    pub groups: Vec<SettingsGroup>
}

impl SettingsMaster {
    pub fn to_ws_class(&self) {
        let mut code = String::from("");

        code += &format!("class {}\n", self.name);
        code += "{\n";

        for group in &self.groups {
            code += &format!("\tpublic var {} : {};\n", group.id, group.to_ws_struct_name(&self.name));
        }

        //TODO function fetching vars from CInGameConfigWrapper

        code += "}\n";
    }
}