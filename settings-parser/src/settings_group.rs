use crate::settings_var::SettingsVar;

#[derive(Default)]
pub struct SettingsGroup {
    pub id: String,
    pub vars: Vec<SettingsVar>
}

impl SettingsGroup {
    pub fn to_ws_struct_name(&self, settings_master_name: &str) -> String {
        format!("{}_{}", settings_master_name, self.id)
    }

    pub fn to_ws_struct(&self, settings_master_name: &str) {
        let mut code = String::from("");

        code += &format!("struct {}\n", self.to_ws_struct_name(settings_master_name));
        code += "{\n";

        for var in &self.vars {
            code += &format!("\tvar {} : {};\n", var.id, var.var_type.to_ws_type_str());
        }

        code += "}\n";
    }
}