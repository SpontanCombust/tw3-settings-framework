use crate::{settings_group::SettingsGroup, to_witcher_script::ToWitcherScript, var_type::VarType};

#[derive(Default)]
pub struct SettingsMaster {
    pub name: String,
    pub groups: Vec<SettingsGroup>
}

const MASTER_BASE_CLASS_NAME: &str = "ISettingsMaster";
const READ_SETTINGS_FUNC_NAME: &str = "ReadSettings";
const READ_SETTING_VALUE_FUNC_NAME: &str = "ReadSettingValue";
const WRITE_SETTINGS_FUNC_NAME: &str = "WriteSettings";
const WRITE_SETTING_VALUE_FUNC_NAME: &str = "WriteSettingValue";

impl ToWitcherScript for SettingsMaster {
    fn ws_type_name(&self) -> String {
        self.name.clone()
    }

    fn ws_code_body(&self) -> String {
        let mut code = String::new();

        code += &format!("// Code generated using Mod Settings Framework & Utilites v{} by SpontanCombust\n\n", option_env!("CARGO_PKG_VERSION").unwrap());

        code += &format!("class {} extends {}\n", self.name, MASTER_BASE_CLASS_NAME);
        code += "{\n";

        code += &settings_class_variables(self);
        
        code += "\n";
        code += &read_settings_function(self);

        code += "\n";
        code += &write_settings_function(self);

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

fn read_settings_function(master: &SettingsMaster) -> String {
    let mut code = String::new();

    code += &format!("\tpublic function {}()\n", READ_SETTINGS_FUNC_NAME);
    code += "\t{\n";

    code += "\t\tvar config : CInGameConfigWrapper;\n";
    code += "\t\tconfig = theGame.GetInGameConfigWrapper();\n";
    code += "\n";

    for group in &master.groups {
        for var in &group.vars {
            let mut get_var_value = format!("{}(config, '{}', '{}')", READ_SETTING_VALUE_FUNC_NAME, group.id, var.id);

            // surround with type cast if necessary
            get_var_value = match &var.var_type {
                VarType::Options | VarType::SliderInt => format!("StringToInt({}, 0)", get_var_value),
                VarType::SliderFloat => format!("StringToFloat({}, 0.0)", get_var_value),
                _ => get_var_value // bools are assigned without explicit cast from string
            };

            code += &format!("\t\t{}.{} = {};\n", group.name, var.name, get_var_value);
        }
        code += "\n";
    }

    code += "\t}\n";

    code
}

fn write_settings_function(master: &SettingsMaster) -> String {
    let mut code = String::new();

    code += &format!("\tpublic function {}()\n", WRITE_SETTINGS_FUNC_NAME);
    code += "\t{\n";

    code += "\t\tvar config : CInGameConfigWrapper;\n";
    code += "\t\tconfig = theGame.GetInGameConfigWrapper();\n";
    code += "\n";

    for group in &master.groups {
        for var in &group.vars {
            let var_value_str = match &var.var_type {
                VarType::Options | VarType::SliderInt => format!("IntToString({}.{})", group.name, var.name),
                VarType::SliderFloat => format!("FloatToString({}.{})", group.name, var.name),
                _ => format!("{}.{}", group.name, var.name) // ShowKnown in hud.ws does no cast for bool either
            };

            code += &format!("\t\t{}(config, '{}', '{}', {});\n", WRITE_SETTING_VALUE_FUNC_NAME, group.id, var.id, var_value_str);
        }
        code += "\n";
    }

    code += "\t\ttheGame.SaveUserSettings();\n";

    code += "\t}\n";

    code
}