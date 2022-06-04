use crate::{settings_group::SettingsGroup, to_witcher_script::ToWitcherScript, var_type::VarType};

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

        code += &format!("// Code generated using Mod Settings Framework & Utilites v{} by SpontanCombust\n\n", option_env!("CARGO_PKG_VERSION").unwrap());

        code += &format!("class {}\n", self.name);
        code += "{\n";

        code += &settings_class_variables(self);
        
        code += "\n";
        code += &update_settings_function(self);

        code += "}\n";

        code
    }
}

fn settings_class_variables(master: &SettingsMaster) -> String {
    let mut code = String::new();

    for group in &master.groups {
        code += &format!("\tpublic var {} : {};\n", group.name, group.ws_type_name());
    }

    code
}

fn update_settings_function(master: &SettingsMaster) -> String {
    let mut code = String::new();

    //TODO rename to ReadSettings() and add WriteSettings()
    code += "\tpublic function UpdateSettings()\n";
    code += "\t{\n";

    code += "\t\tvar config : CInGameConfigWrapper;\n";
    code += "\t\tconfig = theGame.GetInGameConfigWrapper();\n";
    code += "\n";

    for group in &master.groups {
        for var in &group.vars {
            let mut get_var_value = format!("config.GetVarValue('{}', '{}')", group.id, var.id);

            // surround with type cast if necessary
            get_var_value = match &var.var_type {
                VarType::Options | VarType::SliderInt => format!("StringToInt({}, 0)", get_var_value),
                VarType::SliderFloat => format!("StringToFloat({}, 0.0)", get_var_value),
                _ => get_var_value
            };

            code += &format!("\t\t{}.{} = {};\n", group.name, var.name, get_var_value);
        }
        code += "\n";
    }

    code += "\t}\n";

    code
}