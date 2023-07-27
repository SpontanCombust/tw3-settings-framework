use crate::{settings_group::SettingsGroup, traits::{ToWitcherScriptType, WitcherScript}, var_type::VarType, cli::CLI, xml::user_config::UserConfig};

pub struct SettingsMaster {
    pub class_name: String, // name of the class in the WitcherScript
    pub mod_version: String,
    pub groups: Vec<SettingsGroup>,
    pub validate_values: bool
}

impl SettingsMaster {
    pub fn from(xml_user_config: &UserConfig, cli: &CLI) -> Self {       
        let class_name = cli.settings_master_name.clone();
        let mod_version = cli.mod_version.clone();
        let validate_values = !cli.no_var_validation;
        let groups = xml_user_config.groups.iter().map(|g| SettingsGroup::from(g, cli)).collect();
       
        SettingsMaster {
            class_name,
            mod_version,
            groups,
            validate_values,
        }
    }
}




const MASTER_BASE_CLASS_NAME: &str = "ISettingsMaster";
const MASTER_MOD_VERSION_VAR_NAME: &str = "modVersion";
const MASTER_INIT_FUNC_NAME: &str = "Init";
const MASTER_VALIDATE_VALUES_FUNC_NAME: &str = "ValidateSettings";
const MASTER_READ_SETTINGS_FUNC_NAME: &str = "ReadSettings";
const MASTER_READ_SETTING_VALUE_FUNC_NAME: &str = "ReadSettingValue";
const MASTER_WRITE_SETTINGS_FUNC_NAME: &str = "WriteSettings";
const MASTER_WRITE_SETTING_VALUE_FUNC_NAME: &str = "WriteSettingValue";
const MASTER_RESET_SETTINGS_TO_DEFAULT_FUNC_NAME: &str = "ResetSettingsToDefault";
const MASTER_SHOULD_RESET_TO_DEFAULT_ON_INIT_FUNC_NAME: &str = "ShouldResetSettingsToDefaultOnInit";
const GROUP_RESET_SETTINGS_TO_DEFAULT_FUNC_NAME: &str = "ResetToDefault";

impl ToWitcherScriptType for SettingsMaster {
    fn ws_type_name(&self) -> String {
        self.class_name.clone()
    }

    fn ws_type_definition(&self, buffer: &mut WitcherScript) -> bool {
        buffer.push_line(&format!("class {} extends {}", self.class_name, MASTER_BASE_CLASS_NAME));
        buffer.push_indent("{");

        default_variable_values(self, buffer);
        
        buffer.new_line();
        settings_class_variables(self, buffer);

        buffer.new_line();
        init_function(self, buffer);

        if self.validate_values {
            buffer.new_line();
            validate_values_function(self, buffer);
        }
        
        buffer.new_line();
        read_settings_function(self, buffer);

        buffer.new_line();
        write_settings_function(self, buffer);

        buffer.new_line();
        reset_settings_to_default_function(self, buffer);

        buffer.new_line();
        should_reset_to_default_on_init_function(self, buffer);

        buffer.pop_indent("}");

        true
    }
}

fn settings_class_variables(master: &SettingsMaster, buffer: &mut WitcherScript) {
    for group in &master.groups {
        buffer.push_line(&format!("public var {} : {};", group.var_name, group.ws_type_name()));
    }
}

fn default_variable_values(master: &SettingsMaster, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("default {} = \"{}\";", MASTER_MOD_VERSION_VAR_NAME, master.mod_version));
}

fn init_function(master: &SettingsMaster, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("public /* override */ function {}() : void", MASTER_INIT_FUNC_NAME));
    buffer.push_indent("{");

    for group in &master.groups {
        buffer.push_line(&format!("{n} = new {t} in this; {n}.Init(this);", n = group.var_name, t = group.ws_type_name()));
    }

    buffer.new_line();
    buffer.push_line(&format!("super.{}();", MASTER_INIT_FUNC_NAME));

    buffer.pop_indent("}");
}

fn validate_values_function(master: &SettingsMaster, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("public /* override */ function {}() : void", MASTER_VALIDATE_VALUES_FUNC_NAME));
    buffer.push_indent("{");

    for group in &master.groups {
        let mut group_has_validation = false;
        for var in &group.vars {
            let validator = match &var.var_type {
                VarType::Int { min, max } => Some(format!("{g}.{v} = Clamp({g}.{v}, {min}, {max});", g = group.var_name, v = var.var_name)),
                VarType::Float { min, max } => Some(format!("{g}.{v} = ClampF({g}.{v}, {min}, {max});", g = group.var_name, v = var.var_name)),
                VarType::Enum { name, values } => Some(format!("{g}.{v} = ({t})Clamp((int){g}.{v}, {min}, {max});",
                                                                t = name,
                                                                g = group.var_name, 
                                                                v = var.var_name, 
                                                                min = 0, 
                                                                max = values.len() - 1)),
                _ => None,
            };

            if let Some(validator) = validator {
                buffer.push_line(&validator);
                group_has_validation = true;
            }
        }

        if group_has_validation {
            buffer.new_line();
        }
    }

    buffer.push_line(&format!("super.{}();", MASTER_VALIDATE_VALUES_FUNC_NAME));

    buffer.pop_indent("}");
}

fn read_settings_function(master: &SettingsMaster, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("public /* override */ function {}() : void", MASTER_READ_SETTINGS_FUNC_NAME));
    buffer.push_indent("{");

    buffer.push_line("var config : CInGameConfigWrapper;");
    buffer.push_line("config = theGame.GetInGameConfigWrapper();");
    buffer.new_line();

    for group in &master.groups {
        for var in &group.vars {
            let mut get_var_value = format!("{}(config, '{}', '{}')", MASTER_READ_SETTING_VALUE_FUNC_NAME, group.id, var.id);

            // surround with type cast if necessary
            get_var_value = match &var.var_type {
                VarType::Bool => format!("StringToBool({})", get_var_value),
                VarType::Int {..} => format!("StringToInt({}, 0)", get_var_value),
                VarType::Float {..} => format!("StringToFloat({}, 0.0)", get_var_value),
                VarType::Enum { name, .. } => format!("({})StringToInt({}, 0)", name, get_var_value),
            };

            buffer.push_line(&format!("{}.{} = {};", group.var_name, var.var_name, get_var_value));
        }
        buffer.new_line();
    }

    if master.validate_values {
        buffer.push_line(&format!("this.{}();", MASTER_VALIDATE_VALUES_FUNC_NAME));
    }
    buffer.push_line(&format!("super.{}();", MASTER_READ_SETTINGS_FUNC_NAME));

    buffer.pop_indent("}");
}

fn write_settings_function(master: &SettingsMaster, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("public /* override */ function {}() : void", MASTER_WRITE_SETTINGS_FUNC_NAME))
          .push_indent("{");

    buffer.push_line("var config : CInGameConfigWrapper;")
          .push_line("config = theGame.GetInGameConfigWrapper();")
          .new_line();

    if master.validate_values {
        buffer.push_line(&format!("this.{}();", MASTER_VALIDATE_VALUES_FUNC_NAME))
              .new_line();
    }

    for group in &master.groups {
        for var in &group.vars {
            let var_value_str = match var.var_type {
                VarType::Bool => format!("BoolToString({}.{})", group.var_name, var.var_name),
                VarType::Int {..} => format!("IntToString({}.{})", group.var_name, var.var_name),
                VarType::Float {..} => format!("FloatToString({}.{})", group.var_name, var.var_name),
                VarType::Enum {..} => format!("IntToString((int){}.{})", group.var_name, var.var_name),
            };

            buffer.push_line(&format!("{}(config, '{}', '{}', {});", MASTER_WRITE_SETTING_VALUE_FUNC_NAME, group.id, var.id, var_value_str));
        }
        buffer.new_line();
    }

    buffer.push_line(&format!("super.{}();", MASTER_WRITE_SETTINGS_FUNC_NAME));
          
    buffer.pop_indent("}");
}

fn reset_settings_to_default_function(master: &SettingsMaster, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("public /* override */ function {}() : void", MASTER_RESET_SETTINGS_TO_DEFAULT_FUNC_NAME))
          .push_indent("{");

    for group in &master.groups {
        buffer.push_line(&format!("{}.{}();", group.var_name, GROUP_RESET_SETTINGS_TO_DEFAULT_FUNC_NAME));
    }

    buffer.pop_indent("}");
}

fn should_reset_to_default_on_init_function(master: &SettingsMaster, buffer: &mut WitcherScript) {
    let group_id = &master.groups[0].id;
    let var_id = &master.groups[0].vars[0].id;

    buffer.push_line(&format!("public /* override */ function {}() : bool", MASTER_SHOULD_RESET_TO_DEFAULT_ON_INIT_FUNC_NAME))
          .push_indent("{");
    
    buffer.push_line("var config : CInGameConfigWrapper;")
          .push_line("config = theGame.GetInGameConfigWrapper();")
          .new_line()
          .push_line(&format!("return config.GetVarValue('{}','{}') == \"\";", group_id, var_id));
    
    buffer.pop_indent("}");
}