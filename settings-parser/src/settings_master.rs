use crate::{settings_group::SettingsGroup, to_witcher_script::ToWitcherScript};

#[derive(Default)]
pub struct SettingsMaster {
    pub name: String,
    pub groups: Vec<SettingsGroup>
}

impl ToWitcherScript for SettingsMaster {
    fn ws_type_name(&self) -> String {
        self.name.clone()
    }

    fn ws_code_body(&self) -> String {
        let mut code = String::new();

        code += &format!("class {}\n", self.name);
        code += "{\n";

        for group in &self.groups {
            code += &format!("\tpublic var {} : {};\n", group.id, group.ws_type_name());
        }

        //TODO function fetching vars from CInGameConfigWrapper

        code += "}\n";

        code
    }
}