use roxmltree::Node;

use crate::{settings_group::SettingsGroup, traits::{ToWitcherScript, FromXMLNode}, var_type::VarType, cli::CLI, utils::{validate_name, is_integral_range}};

pub struct SettingsMaster {
    pub class_name: String,
    pub mod_version: String,
    pub groups: Vec<SettingsGroup>,
    pub validate_values: bool
}

impl FromXMLNode for SettingsMaster {
    fn from_xml_node(node: &Node, cli: &CLI) -> Result<Option<Self>, String> {
        let tag_name = node.tag_name().name();
        if tag_name != "UserConfig" {
            return Err(format!("Wrong XML node. Expected UserConfig, received {}", tag_name))
        }

        if let Err(err) = validate_name(&cli.settings_master_name) {
            return Err(format!("Invalid settings master name: {}", err));
        }
        
        let class_name = cli.settings_master_name.clone();
        let mod_version = cli.mod_version.clone();
        let validate_values = !cli.no_var_validation;
        let mut groups = Vec::<SettingsGroup>::new();
    
        let group_nodes: Vec<Node> = node.children().filter(|n| n.has_tag_name("Group")).collect();

        if group_nodes.is_empty() {
            return Err("No Groups found inside UserConfig".to_string());
        }
        
        for group_node in &group_nodes {
            match SettingsGroup::from_xml_node(group_node, cli) {
                Ok(group_opt) => {
                    if let Some(group) = group_opt {
                        groups.push(group);
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
            
        return Ok(Some(SettingsMaster {
            class_name,
            mod_version,
            groups,
            validate_values,
        }));
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

impl ToWitcherScript for SettingsMaster {
    fn ws_type_name(&self) -> String {
        self.class_name.clone()
    }

    fn ws_type_definition(&self, buffer: &mut String) -> bool {
        buffer.push_str(&format!("class {} extends {}\n", self.class_name, MASTER_BASE_CLASS_NAME));
        buffer.push_str("{\n");

        default_variable_values(self, buffer);
        
        buffer.push_str("\n");
        settings_class_variables(self, buffer);

        buffer.push_str("\n");
        init_function(self, buffer);

        if self.validate_values {
            buffer.push_str("\n");
            validate_values_function(self, buffer);
        }
        
        buffer.push_str("\n");
        read_settings_function(self, buffer);

        buffer.push_str("\n");
        write_settings_function(self, buffer);

        buffer.push_str("\n");
        reset_settings_to_default_function(self, buffer);

        buffer.push_str("\n");
        should_reset_to_default_on_init_function(self, buffer);

        buffer.push_str("}\n");

        true
    }
}

fn settings_class_variables(master: &SettingsMaster, buffer: &mut String) {
    for group in &master.groups {
        buffer.push_str(&format!("\tpublic var {} : {};\n", group.var_name, group.ws_type_name()));
    }
}

fn default_variable_values(master: &SettingsMaster, buffer: &mut String) {
    buffer.push_str(&format!("\tdefault {} = \"{}\";\n", MASTER_MOD_VERSION_VAR_NAME, master.mod_version));
}

fn init_function(master: &SettingsMaster, buffer: &mut String) {
    buffer.push_str(&format!("\tpublic /* override */ function {}() : void\n", MASTER_INIT_FUNC_NAME));
    buffer.push_str("\t{\n");

    for group in &master.groups {
        buffer.push_str(&format!("\t\t{} = new {} in this; ", group.var_name, group.ws_type_name()));
        buffer.push_str(&format!("{}.Init(this);\n", group.var_name));
    }

    buffer.push_str("\n");
    buffer.push_str(&format!("\t\tsuper.{}();\n", MASTER_INIT_FUNC_NAME));

    buffer.push_str("\t}\n");
}

fn validate_values_function(master: &SettingsMaster, buffer: &mut String) {
    buffer.push_str(&format!("\tpublic /* override */ function {}() : void\n", MASTER_VALIDATE_VALUES_FUNC_NAME));
    buffer.push_str("\t{\n");

    for group in &master.groups {
        let mut group_has_validation = false;
        for var in &group.vars {
            let validator = match &var.var_type {
                VarType::Options { options_array, .. } => Some(format!("{g}.{v} = Clamp({g}.{v}, {min}, {max});", 
                                                                         g = group.var_name, 
                                                                         v = var.var_name, 
                                                                         min = 0, 
                                                                         max = options_array.len() - 1)),
                VarType::Slider { min, max, div } => Some(format!("{g}.{v} = {func}({g}.{v}, {min}, {max});",
                                                                    g = group.var_name,
                                                                    v = var.var_name,
                                                                    func = if is_integral_range(*min, *max, *div) { "Clamp" } else { "ClampF" })),
                _ => None
            };

            if let Some(validator) = validator {
                buffer.push_str(&format!("\t\t{}\n", validator));
                group_has_validation = true;
            }
        }

        if group_has_validation {
            buffer.push_str("\n");
        }
    }

    buffer.push_str("\n");
    buffer.push_str(&format!("\t\tsuper.{}();\n", MASTER_VALIDATE_VALUES_FUNC_NAME));

    buffer.push_str("\t}\n");
}

fn read_settings_function(master: &SettingsMaster, buffer: &mut String) {
    buffer.push_str(&format!("\tpublic /* override */ function {}() : void\n", MASTER_READ_SETTINGS_FUNC_NAME));
    buffer.push_str("\t{\n");

    buffer.push_str("\t\tvar config : CInGameConfigWrapper;\n");
    buffer.push_str("\t\tconfig = theGame.GetInGameConfigWrapper();\n");
    buffer.push_str("\n");

    for group in &master.groups {
        for var in &group.vars {
            let mut get_var_value = format!("{}(config, '{}', '{}')", MASTER_READ_SETTING_VALUE_FUNC_NAME, group.id, var.id);

            // surround with type cast if necessary
            get_var_value = match var.var_type {
                VarType::Options {..} => format!("StringToInt({}, 0)", get_var_value),
                VarType::Slider { min, max, div } => {
                    if is_integral_range(min, max, div) {
                        format!("StringToInt({}, 0)", get_var_value)
                    } else {
                        format!("StringToFloat({}, 0.0)", get_var_value)
                    }
                }
                VarType::Toggle => format!("StringToBool({})", get_var_value),
            };

            buffer.push_str(&format!("\t\t{}.{} = {};\n", group.var_name, var.var_name, get_var_value));
        }
        buffer.push_str("\n");
    }

    buffer.push_str("\n");
    if master.validate_values {
        buffer.push_str(&format!("\t\tthis.{}();\n", MASTER_VALIDATE_VALUES_FUNC_NAME));
    }
    buffer.push_str(&format!("\t\tsuper.{}();\n", MASTER_READ_SETTINGS_FUNC_NAME));

    buffer.push_str("\t}\n");
}

fn write_settings_function(master: &SettingsMaster, buffer: &mut String) {
    buffer.push_str(&format!("\tpublic /* override */ function {}() : void\n", MASTER_WRITE_SETTINGS_FUNC_NAME));
    buffer.push_str("\t{\n");

    buffer.push_str("\t\tvar config : CInGameConfigWrapper;\n");
    buffer.push_str("\t\tconfig = theGame.GetInGameConfigWrapper();\n");
    buffer.push_str("\n");

    if master.validate_values {
        buffer.push_str(&format!("\t\tthis.{}();\n", MASTER_VALIDATE_VALUES_FUNC_NAME));
        buffer.push_str("\n");
    }

    for group in &master.groups {
        for var in &group.vars {
            let var_value_str = match var.var_type {
                VarType::Options {..} => format!("IntToString({}.{})", group.var_name, var.var_name),
                VarType::Slider { min, max, div } => {
                    if is_integral_range(min, max, div) {
                        format!("IntToString({}.{})", group.var_name, var.var_name)
                    } else {
                        format!("FloatToString({}.{})", group.var_name, var.var_name)
                    }
                }
                VarType::Toggle => format!("BoolToString({}.{})", group.var_name, var.var_name),
            };

            buffer.push_str(&format!("\t\t{}(config, '{}', '{}', {});\n", MASTER_WRITE_SETTING_VALUE_FUNC_NAME, group.id, var.id, var_value_str));
        }
        buffer.push_str("\n");
    }

    buffer.push_str("\n");
    buffer.push_str(&format!("\t\tsuper.{}();\n", MASTER_WRITE_SETTINGS_FUNC_NAME));

    buffer.push_str("\t}\n");
}

fn reset_settings_to_default_function(master: &SettingsMaster, buffer: &mut String) {
    buffer.push_str(&format!("\tpublic /* override */ function {}() : void\n", MASTER_RESET_SETTINGS_TO_DEFAULT_FUNC_NAME));
    buffer.push_str("\t{\n");

    for group in &master.groups {
        buffer.push_str(&format!("\t\t{}.{}();\n", group.var_name, GROUP_RESET_SETTINGS_TO_DEFAULT_FUNC_NAME));
    }

    buffer.push_str("\t}\n");
}

fn should_reset_to_default_on_init_function(master: &SettingsMaster, buffer: &mut String) {
    buffer.push_str(&format!("\tpublic /* override */ function {}() : bool\n", MASTER_SHOULD_RESET_TO_DEFAULT_ON_INIT_FUNC_NAME));
    buffer.push_str("\t{\n");
    
    buffer.push_str("\t\tvar config : CInGameConfigWrapper;\n");
    buffer.push_str("\t\tconfig = theGame.GetInGameConfigWrapper();\n");
    buffer.push_str("\n");
     
    let group_id = &master.groups[0].id;
    let var_id = &master.groups[0].vars[0].id;

    buffer.push_str(&format!("\t\treturn config.GetVarValue('{}','{}') == \"\";\n", group_id, var_id));
    
    buffer.push_str("\t}\n");
}