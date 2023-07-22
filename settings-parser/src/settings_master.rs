use roxmltree::{Document, Node};

use crate::{settings_group::SettingsGroup, to_witcher_script::ToWitcherScript, var_type::VarType, cli::CLI, utils::validate_name};

#[derive(Default)]
pub struct SettingsMaster {
    pub name: String,
    pub mod_version: String,
    pub groups: Vec<SettingsGroup>,
    pub validate_values: bool
}

impl SettingsMaster {
    pub fn from_xml(xml_text: String, cli: &CLI) -> Result<SettingsMaster, String> {
        if let Err(err) = validate_name(&cli.settings_master_name) {
            return Err(format!("Invalid settings master name: {}", err));
        }
    
        let doc = match Document::parse(&xml_text) {
            Ok(doc) => doc,
            Err(err) => {
                return Err(err.to_string())
            }
        };
        
        let mut master = SettingsMaster::default();
        master.name = cli.settings_master_name.clone();
        master.mod_version = cli.mod_version.clone();
        master.validate_values = !cli.no_var_validation;
    
        if let Some(root_node) = doc.descendants().find(|n| n.has_tag_name("UserConfig")) {
            let group_nodes: Vec<Node> = root_node.children().filter(|n| n.has_tag_name("Group")).collect();
    
            if group_nodes.is_empty() {
                return Err("No Groups found inside UserConfig".to_string());
            }
            
            for group_node in &group_nodes {
                match SettingsGroup::from_xml(group_node, cli) {
                    Ok(group_opt) => {
                        if let Some(group) = group_opt {
                            master.groups.push(group);
                        }
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
        }
        else {
            return Err("No UserConfig root node found".to_string());
        }
    
        return Ok(master);
    }
}




const MASTER_BASE_CLASS_NAME: &str = "ISettingsMaster";
const MASTER_MOD_VERSION_VAR_NAME: &str = "modVersion";
const MASTER_INIT_FUNC_NAME: &str = "Init";
const MASTER_VALIDATE_VALUES_FUNC_NAME: &str = "ValidateValues";
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

        code += &format!("// Code generated using Mod Settings Framework v{} by SpontanCombust & Aeltoth\n\n", option_env!("CARGO_PKG_VERSION").unwrap());

        code += &format!("class {} extends {}\n", self.name, MASTER_BASE_CLASS_NAME);
        code += "{\n";

        code += &default_variable_values(self);
        
        code += "\n";
        code += &settings_class_variables(self);

        code += "\n";
        code += &init_function(self);

        if self.validate_values {
            code += "\n";
            code += &validate_values_function(self);
        }
        
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

    code += &format!("\tpublic /* override */ function {}() : void\n", MASTER_INIT_FUNC_NAME);
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

fn validate_values_function(master: &SettingsMaster) -> String {
    let mut code = String::new();

    code += &format!("\tpublic /* override */ function {}() : void\n", MASTER_VALIDATE_VALUES_FUNC_NAME);
    code += "\t{\n";

    for group in &master.groups {
        let mut group_code = String::new();
        for var in &group.vars {
            let validator = match &var.var_type {
                VarType::Options(options)   => Some(format!("{g}.{v} = Clamp({g}.{v}, {min}, {max});", 
                                                        g = group.name, 
                                                        v = var.name, 
                                                        min = 0, 
                                                        max = options.options_array.len() - 1)),
                VarType::Slider(slider)     => Some(format!("{g}.{v} = {func}({g}.{v}, {min}, {max});",
                                                        g = group.name,
                                                        v = var.name,
                                                        func = if slider.is_integral() { "Clamp" } else { "ClampF" },
                                                        min = slider.min,
                                                        max = slider.max)),
                _ => None
            };

            if let Some(validator) = validator {
                group_code += &format!("\t\t{}\n", validator);
            }
        }

        if !group_code.is_empty() {
            code += &format!("{}\n", group_code);
        }
    }

    code += "\n";
    code += &format!("\t\tsuper.{}();\n", MASTER_VALIDATE_VALUES_FUNC_NAME);

    code += "\t}\n";

    return code;
}

fn read_settings_function(master: &SettingsMaster) -> String {
    let mut code = String::new();

    code += &format!("\tpublic /* override */ function {}() : void\n", MASTER_READ_SETTINGS_FUNC_NAME);
    code += "\t{\n";

    code += "\t\tvar config : CInGameConfigWrapper;\n";
    code += "\t\tconfig = theGame.GetInGameConfigWrapper();\n";
    code += "\n";

    for group in &master.groups {
        for var in &group.vars {
            let mut get_var_value = format!("{}(config, '{}', '{}')", MASTER_READ_SETTING_VALUE_FUNC_NAME, group.id, var.id);

            // surround with type cast if necessary
            get_var_value = match &var.var_type {
                VarType::Options(_) => format!("StringToInt({}, 0)", get_var_value),
                VarType::Slider(slider) => {
                    if slider.is_integral() {
                        format!("StringToInt({}, 0)", get_var_value)
                    } else {
                        format!("StringToFloat({}, 0.0)", get_var_value)
                    }
                }
                VarType::Toggle => format!("StringToBool({})", get_var_value),
            };

            code += &format!("\t\t{}.{} = {};\n", group.name, var.name, get_var_value);
        }
        code += "\n";
    }

    code += "\n";
    if master.validate_values {
        code += &format!("\t\tthis.{}();\n", MASTER_VALIDATE_VALUES_FUNC_NAME);
    }
    code += &format!("\t\tsuper.{}();\n", MASTER_READ_SETTINGS_FUNC_NAME);

    code += "\t}\n";

    return code;
}

fn write_settings_function(master: &SettingsMaster) -> String {
    let mut code = String::new();

    code += &format!("\tpublic /* override */ function {}() : void\n", MASTER_WRITE_SETTINGS_FUNC_NAME);
    code += "\t{\n";

    code += "\t\tvar config : CInGameConfigWrapper;\n";
    code += "\t\tconfig = theGame.GetInGameConfigWrapper();\n";
    code += "\n";

    if master.validate_values {
        code += &format!("\t\tthis.{}();\n", MASTER_VALIDATE_VALUES_FUNC_NAME);
        code += "\n";
    }

    for group in &master.groups {
        for var in &group.vars {
            let var_value_str = match &var.var_type {
                VarType::Options(_) => format!("IntToString({}.{})", group.name, var.name),
                VarType::Slider(slider) => {
                    if slider.is_integral() {
                        format!("IntToString({}.{})", group.name, var.name)
                    } else {
                        format!("FloatToString({}.{})", group.name, var.name)
                    }
                }
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

    code += &format!("\tpublic /* override */ function {}() : void\n", MASTER_RESET_SETTINGS_TO_DEFAULT_FUNC_NAME);
    code += "\t{\n";

    for group in &master.groups {
        code += &format!("\t\t{}.{}();\n", group.name, GROUP_RESET_SETTINGS_TO_DEFAULT_FUNC_NAME);
    }

    code += "\t}\n";

    return code;
}

fn should_reset_to_default_on_init_function(master: &SettingsMaster) -> String {
    let mut code = String::new();
    code += &format!("\tpublic /* override */ function {}() : bool\n", MASTER_SHOULD_RESET_TO_DEFAULT_ON_INIT_FUNC_NAME);
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