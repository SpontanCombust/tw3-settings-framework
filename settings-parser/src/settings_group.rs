use crate::{settings_var::SettingsVar, to_witcher_script::ToWitcherScript};

#[derive(Default)]
pub struct SettingsGroup {
    pub master_name: String,
    pub id: String,
    pub name: String,
    pub vars: Vec<SettingsVar>
}

const SETTINGS_GROUP_PARENT_CLASS: &str = "ISettingsGroup";

impl ToWitcherScript for SettingsGroup {
    fn ws_type_name(&self) -> String {
        format!("{}_{}", self.master_name, self.name)
    }

    fn ws_code_body(&self) -> String {
        let mut code = String::new();

        code += &format!("class {} extends {}\n", self.ws_type_name(), SETTINGS_GROUP_PARENT_CLASS);
        code += "{\n";

        for var in &self.vars {
            code += &format!("\tpublic {};\n", var.ws_code_body());
        }

        code += "}\n";

        code
    }
}