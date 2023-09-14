use std::collections::HashMap;

use crate::{
    settings_group::SettingsGroup, 
    traits::{WitcherScriptType, WitcherScript, WitcherScriptTypeDef}, 
    settings_var_type::SettingsVarType, 
    cli::CLI, xml::user_config::UserConfig, 
    settings_enum::{
        SettingsEnum, 
        SettingsEnumValueMapping
    }, 
    constants::{
        MASTER_BASE_CLASS_NAME, 
        MASTER_MOD_VERSION_VAR_NAME, 
        MASTER_INIT_PARSER_FUNC_NAME,
        MASTER_SHOULD_RESET_TO_DEFAULT_ON_INIT_PARSER_FUNC_NAME, 
        MASTER_GROUP_ARRAY_VAR_NAME, 
        MASTER_READ_SETTING_VALUE_FUNC_NAME,
    }
};

pub struct SettingsMaster {
    pub class_name: String, // name of the class in the WitcherScript
    pub mod_version: String,
    pub groups: Vec<SettingsGroup>,
    pub enums: Vec<SettingsEnum>,
    pub validate_values: bool,
    pub generate_getter: bool
}

impl SettingsMaster {
    pub fn try_from(xml_user_config: UserConfig, cli: &CLI) -> Result<Self, String> {  
        let class_name = xml_user_config.class_name;
        let mod_version = xml_user_config.mod_version.unwrap_or("1.0".into());
        let validate_values = xml_user_config.validate.unwrap_or(true);
        let generate_getter = !cli.no_getter;
        
        let mut settings_groups = Vec::new();
        for group in xml_user_config.groups.iter() {
            if !group.ignore.unwrap_or(false) {
                settings_groups.push(SettingsGroup::try_from(&group, &class_name, &xml_user_config.mod_prefixes, validate_values)?);
            }
        }

        let mut settings_master = SettingsMaster {
            class_name,
            mod_version,
            groups: settings_groups,
            enums: Vec::new(),
            validate_values,
            generate_getter
        };

        let needs_enum_value_mappings = settings_master.fetch_enums(cli)?;

        if needs_enum_value_mappings {
            settings_master.create_enum_value_mappings();
        }
  
        Ok(settings_master)
    }

    // Returns whether enum value mapping is needed
    fn fetch_enums(&mut self, cli: &CLI) -> Result<bool, String> {
        // convenience struct that also contains group id and var id of the options var that enum was made from
        struct EnumData<'a> {
            group_id: &'a str,
            var_id: &'a str,
            val: &'a SettingsEnum
        }

        let mut enum_type_map: HashMap<&str, Vec<EnumData>> = HashMap::new();
        for sg in self.groups.iter() {
            for sv in sg.vars.iter() {
                if let SettingsVarType::Enum { val, .. } = &sv.var_type {
                    let prefix = val.type_name.as_str();
                    let data = EnumData {
                        group_id: sg.id.as_str(),
                        var_id: sv.id.as_str(),
                        val
                    };

                    if let Some(data_vec) = enum_type_map.get_mut(prefix) {
                        data_vec.push(data);
                    } else {
                        enum_type_map.insert(prefix, vec![data]);
                    }
                }
            }
        }

        if enum_type_map.is_empty() {
            return Ok(false);
        }

        let mut enum_value_mapping_needed = false;
        for (_, data_vec) in enum_type_map.iter() {
            if data_vec.is_empty() {
                continue;
            } else if data_vec.len() == 1 {
                self.enums.push(data_vec[0].val.clone());
                continue;
            }
            
            // an enum type that will contain all possible values from OptionsArrays with the same common prefix 
            let mut unified_enum = data_vec[0].val.clone();
            for data in data_vec.iter().skip(1) {
                // comparison concerns not only the values inside, but how they are ordered
                if unified_enum.values != data.val.values {
                    if cli.strict_enums {
                        return Err(format!("Some OptionArrays have the same common prefix, but different sets of values. See {}::{} and {}::{}",
                                            data_vec[0].group_id, data_vec[0].var_id, data.group_id, data.var_id));
                    }

                    enum_value_mapping_needed = true;

                    for value in &data.val.values {
                        if !unified_enum.values.contains(value) {
                            unified_enum.values.push(value.clone());
                        }
                    }
                }
            }

            self.enums.push(unified_enum);
        }

        // enum definitions kept being parsed in different order between program runs
        self.enums.sort_by(|e1, e2| e1.type_name.cmp(&e2.type_name));

        Ok(enum_value_mapping_needed)
    }

    fn create_enum_value_mappings(&mut self) {
        let it = self.groups.iter_mut()
        .flat_map(|g| &mut g.vars)
        .map(|v| &mut v.var_type)
        .filter_map(|vt| {
            if let SettingsVarType::Enum { val, val_mapping } = vt {
                Some((val, val_mapping))
            } else {
                None
            }
        });
        
        for (val, val_mapping) in it {
            let mut mapping = SettingsEnumValueMapping::default();
            mapping.resize(val.values.len(), 0);

            let unified_enum = self.enums.iter()
                               .find(|e| e.type_name == val.type_name)
                               .expect(&format!("Unified enum type not found for enum {}", val.type_name));

            // no need for any specialized mapping if those types are exactly the same
            if val == unified_enum {
                *val_mapping = None;
                continue;
            }

            for i in 0..val.values.len() {
                for j in 0..unified_enum.values.len() {
                    if val.values[i] == unified_enum.values[j] {
                        mapping[i] = j;
                        break;
                    }
                }
            }

            *val_mapping = Some(mapping);
        }
    }
}




impl WitcherScriptType for SettingsMaster {
    fn ws_type_name(&self) -> String {
        self.class_name.clone()
    }
}

impl WitcherScriptTypeDef for SettingsMaster {
    fn ws_type_definition(&self, buffer: &mut WitcherScript) {
        buffer.push_line(&format!("class {} extends {}", self.class_name, MASTER_BASE_CLASS_NAME));
        buffer.push_line("{").push_indent();
    
        default_variable_values(self, buffer);
        
        buffer.new_line();
        settings_class_variables(self, buffer);
    
        buffer.new_line();
        init_function(self, buffer);
    
        buffer.new_line();
        should_reset_to_default_on_init_function(self, buffer);
    
        buffer.pop_indent().push_line("}");
        buffer.new_line();

        for g in &self.groups {
            g.ws_type_definition(buffer);
            buffer.new_line();
        }

        for e in &self.enums {
            e.ws_type_definition(buffer);
            buffer.new_line();
        }

        if self.generate_getter {
            buffer.new_line();
            getter_convenience_function(self, buffer);
        }
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
    buffer.push_line(&format!("protected /* override */ function {}() : void", MASTER_INIT_PARSER_FUNC_NAME));
    buffer.push_line("{").push_indent();

    for i in 0..master.groups.len() {
        let group = &master.groups[i];
        buffer.push_line(&format!("{} = new {} in this;", group.var_name, group.ws_type_name()));
        buffer.push_line(&format!("{}.Init(this);", group.var_name));
        buffer.push_line(&format!("{}.PushBack({});", MASTER_GROUP_ARRAY_VAR_NAME, group.var_name));

        if i != master.groups.len() - 1 {
            buffer.new_line();
        }
    }

    buffer.pop_indent().push_line("}");
}

fn should_reset_to_default_on_init_function(master: &SettingsMaster, buffer: &mut WitcherScript) {
    let group_id = &master.groups[0].id;
    let var = &master.groups[0].vars[0];
    let var_id = &var.id;
    let var_not_found_value = &var.var_not_found_value;

    buffer.push_line(&format!("protected /* override */ function {}(config : CInGameConfigWrapper) : bool", MASTER_SHOULD_RESET_TO_DEFAULT_ON_INIT_PARSER_FUNC_NAME))
          .push_line("{").push_indent();
    
    buffer.push_line(&format!("return {}(config, '{}','{}') == \"{}\";", MASTER_READ_SETTING_VALUE_FUNC_NAME, group_id, var_id, var_not_found_value));         
    
    buffer.pop_indent().push_line("}");
}

fn getter_convenience_function(master: &SettingsMaster, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("function Get{m}() : {m}", m=master.class_name))
          .push_line("{").push_indent()
          .push_line(&format!("var settings: {};", master.class_name))
          .new_line()
          .push_line(&format!("settings = ({m})GetSettingsMasterRegistry().GetSettings('{m}');", m=master.class_name))
          .push_line("if(!settings)")
          .push_line("{").push_indent()
          .push_line(&format!("settings = new {} in theGame;", master.class_name))
          .push_line(&format!("GetSettingsMasterRegistry().AddSettings(settings, '{}');", master.class_name))
          .pop_indent().push_line("}")
          .new_line()
          .push_line("return settings;")  
          .pop_indent().push_line("}");  
}  