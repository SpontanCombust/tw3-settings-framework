use crate::{settings_var::SettingsVar, to_witcher_script::ToWitcherScript};

#[derive(Default)]
pub struct SettingsGroup {
    pub master_name: String,
    pub id: String,
    pub vars: Vec<SettingsVar>
}


impl ToWitcherScript for SettingsGroup {
    fn ws_type_name(&self) -> String {
        format!("{}_{}", self.master_name, self.id)
    }

    fn ws_code_body(&self) -> String {
        let mut code = String::from("");

        code += &format!("struct {}\n", self.ws_type_name());
        code += "{\n";

        for var in &self.vars {
            code += &format!("\t{};\n", var.ws_code_body());
        }

        code += "}\n";

        code
    }
}