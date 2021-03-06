use crate::{settings_group::SettingsGroup, to_witcher_script::ToWitcherScript, var_type::VarType};

#[derive(Default)]
pub struct SettingsMaster {
    pub name: String,
    pub mod_version: String,
    pub groups: Vec<SettingsGroup>
}

const MASTER_BASE_CLASS_NAME: &str = "ISettingsMaster";
const MASTER_MOD_VERSION_VAR_NAME: &str = "modVersion";
const MASTER_INIT_FUNC_NAME: &str = "Init";
const MASTER_READ_SETTINGS_FUNC_NAME: &str = "ReadSettings";
const MASTER_READ_SETTING_VALUE_FUNC_NAME: &str = "ReadSettingValue";
const MASTER_WRITE_SETTINGS_FUNC_NAME: &str = "WriteSettings";
const MASTER_WRITE_SETTING_VALUE_FUNC_NAME: &str = "WriteSettingValue";
const MASTER_RESET_SETTINGS_TO_DEFAULT_FUNC_NAME: &str = "ResetSettingsToDefault";
const MASTER_SHOULD_RESET_TO_DEFAULT_ON_INIT_FUNC_NAME: &str = "ShouldResetSettingsToDefaultOnInit";
const GROUP_RESET_SETTINGS_TO_DEFAULT_FUNC_NAME: &str = "ResetToDefault";

impl ToWitcherScript for SettingsMaster {
    fn ws_type_name(&self) -> String {
        self.name.clone()
    }

    fn ws_code_body(&self) -> String {
        let mut code = String::new();

        code += &format!("// Code generated using Mod Settings Framework & Utilites v{} by SpontanCombust\n\n", option_env!("CARGO_PKG_VERSION").unwrap());

        code += &format!("class {} extends {}\n", self.name, MASTER_BASE_CLASS_NAME);
        code += "{\n";

        code += &default_variable_values(self);
        
        code += "\n";
        code += &settings_class_variables(self);

        code += "\n";
        code += &init_function(self);
        
        code += "\n";
        code += &read_settings_function(self);

        code += "\n";
        code += &write_settings_function(self);

        code += "\n";
        code += &reset_settings_to_default_function(self);

        code += "\n";
        code += &should_reset_to_default_on_init_function(self);

        code += "}\n";

        code
    }
}

fn settings_class_variables(master: &SettingsMaster) -> String {
    let mut code = String::new();

    for group in &master.groups {
        code += &format!("\tpublic var {} : {};\n", group.name, group.ws_type_name());
    }

    return code;
}

fn default_variable_values(master: &SettingsMaster) -> String {
    let mut code = String::new();

    code += &format!("\tdefault {} = \"{}\";\n", MASTER_MOD_VERSION_VAR_NAME, master.mod_version);

    return code;
}

fn init_function(master: &SettingsMaster) -> String {
    let mut code = String::new();

    code += &format!("\tpublic function {}() : void\n", MASTER_INIT_FUNC_NAME);
    code += "\t{\n";

    for group in &master.groups {
        code += &format!("\t\t{} = new {} in this; ", group.name, group.ws_type_name());
        code += &format!("{}.Init(this);\n", group.name);
    }

    code += "\n";
    code += &format!("\t\tsuper.{}();\n", MASTER_INIT_FUNC_NAME);

    code += "\t}\n";

    return code;
}

fn read_settings_function(master: &SettingsMaster) -> String {
    let mut code = String::new();

    code += &format!("\tpublic function {}() : void\n", MASTER_READ_SETTINGS_FUNC_NAME);
    code += "\t{\n";

    code += "\t\tvar config : CInGameConfigWrapper;\n";
    code += "\t\tconfig = theGame.GetInGameConfigWrapper();\n";
    code += "\n";

    for group in &master.groups {
        for var in &group.vars {
            let mut get_var_value = format!("{}(config, '{}', '{}')", MASTER_READ_SETTING_VALUE_FUNC_NAME, group.id, var.id);

            // surround with type cast if necessary
            get_var_value = match &var.var_type {
                VarType::Options | VarType::SliderInt => format!("StringToInt({}, 0)", get_var_value),
                VarType::SliderFloat => format!("StringToFloat({}, 0.0)", get_var_value),
                VarType::Toggle => format!("StringToBool({})", get_var_value),
            };

            code += &format!("\t\t{}.{} = {};\n", group.name, var.name, get_var_value);
        }
        code += "\n";
    }

    code += "\n";
    code += &format!("\t\tsuper.{}();\n", MASTER_READ_SETTINGS_FUNC_NAME);

    code += "\t}\n";

    return code;
}

fn write_settings_function(master: &SettingsMaster) -> String {
    let mut code = String::new();

    code += &format!("\tpublic function {}() : void\n", MASTER_WRITE_SETTINGS_FUNC_NAME);
    code += "\t{\n";

    code += "\t\tvar config : CInGameConfigWrapper;\n";
    code += "\t\tconfig = theGame.GetInGameConfigWrapper();\n";
    code += "\n";

    for group in &master.groups {
        for var in &group.vars {
            let var_value_str = match &var.var_type {
                VarType::Options | VarType::SliderInt => format!("IntToString({}.{})", group.name, var.name),
                VarType::SliderFloat => format!("FloatToString({}.{})", group.name, var.name),
                VarType::Toggle => format!("BoolToString({}.{})", group.name, var.name),
            };

            code += &format!("\t\t{}(config, '{}', '{}', {});\n", MASTER_WRITE_SETTING_VALUE_FUNC_NAME, group.id, var.id, var_value_str);
        }
        code += "\n";
    }

    code += "\n";
    code += &format!("\t\tsuper.{}();\n", MASTER_WRITE_SETTINGS_FUNC_NAME);

    code += "\t}\n";

    return code;
}

fn reset_settings_to_default_function(master: &SettingsMaster) -> String {
    let mut code = String::new();

    code += &format!("\tpublic function {}() : void\n", MASTER_RESET_SETTINGS_TO_DEFAULT_FUNC_NAME);
    code += "\t{\n";

    for group in &master.groups {
        code += &format!("\t\t{}.{}();\n", group.name, GROUP_RESET_SETTINGS_TO_DEFAULT_FUNC_NAME);
    }

    code += "\t}\n";

    return code;
}

fn should_reset_to_default_on_init_function(master: &SettingsMaster) -> String {
    let mut code = String::new();
    code += &format!("\tpublic function {}() : bool\n", MASTER_SHOULD_RESET_TO_DEFAULT_ON_INIT_FUNC_NAME);
    code += "\t{\n";
    
    code += "\t\tvar config : CInGameConfigWrapper;\n";
    code += "\t\tconfig = theGame.GetInGameConfigWrapper();\n";
    code += "\n";
     
    let group_id = &master.groups[0].id;
    let var_id = &master.groups[0].vars[0].id;

    code += &format!("\t\treturn config.GetVarValue('{}','{}') == \"\";\n", group_id, var_id);
    
    code += "\t}\n";

    return code;
}