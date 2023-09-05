use crate::settings_var_type::SettingsVarType;


pub const GROUP_PARENT_CLASS: &str = "ISettingsGroup";
pub const GROUP_PARENT_MASTER_VAR_NAME: &str = "m_parentMaster";
pub const GROUP_ID_VAR_NAME: &str = "id";
pub const GROUP_DEFAULT_PRESET_VAR_NAME: &str = "defaultPresetIndex";
pub const GROUP_VALIDATE_VALUES_FUNC_NAME: &str = "ValidateSettings";
pub const GROUP_READ_SETTINGS_FUNC_NAME: &str = "ReadSettings";
pub const GROUP_WRITE_SETTINGS_FUNC_NAME: &str = "WriteSettings";
pub const GROUP_RESET_SETTINGS_TO_DEFAULT_FUNC_NAME: &str = "ResetSettingsToDefault";
pub const GROUP_ENUM_MAPPING_VALIDATE_FUNC_NAME: &str = "EnumValueMappingValidateUnified";
pub const GROUP_VALIDATE_VALUES_PARSER_FUNC_NAME: &str = "Parser_ValidateSettings";
pub const GROUP_READ_SETTINGS_PARSER_FUNC_NAME: &str = "Parser_ReadSettings";
pub const GROUP_WRITE_SETTINGS_PARSER_FUNC_NAME: &str = "Parser_WriteSettings";
pub const GROUP_ENUM_MAPPING_CONFIG_TO_UNIFIED_PARSER_FUNC_NAME: &str = "Parser_EnumValueMappingConfigToUnified";
pub const GROUP_ENUM_MAPPING_UNIFIED_TO_CONFIG_PARSER_FUNC_NAME: &str = "Parser_EnumValueMappingUnifiedToConfig";
pub const GROUP_ENUM_MAPPING_VALIDATE_PARSER_FUNC_NAME: &str = "Parser_EnumValueMappingValidateUnified";

pub const MASTER_BASE_CLASS_NAME: &str = "ISettingsMaster";
pub const MASTER_MOD_VERSION_VAR_NAME: &str = "modVersion";
pub const MASTER_GROUP_ARRAY_VAR_NAME: &str = "m_groups";
pub const MASTER_READ_SETTING_VALUE_FUNC_NAME: &str = "ReadSettingValue";
pub const MASTER_WRITE_SETTING_VALUE_FUNC_NAME: &str = "WriteSettingValue";
pub const MASTER_INIT_PARSER_FUNC_NAME: &str = "Parser_Init";
pub const MASTER_SHOULD_RESET_TO_DEFAULT_ON_INIT_PARSER_FUNC_NAME: &str = "Parser_ShouldResetSettingsToDefaultOnInit";

pub trait ReadSettingValueFnName {
    fn read_setting_value_fn(&self) -> &'static str;
}

impl ReadSettingValueFnName for SettingsVarType {
    fn read_setting_value_fn(&self) -> &'static str {
        match self {
            SettingsVarType::Bool => "ReadBoolSettingValue",
            SettingsVarType::Int {..} => "ReadIntSettingValue",
            SettingsVarType::Float {..} => "ReadFloatSettingValue",
            SettingsVarType::Enum { val_mapping, .. } => {
                if val_mapping.is_some() {
                    "ReadUnifiedEnumSettingValue"
                } else {
                    "ReadIntSettingValue"
                }
            }
        }
    }
}

pub trait WriteSettingValueFnName {
    fn write_setting_value_fn(&self) -> &'static str;
}

impl WriteSettingValueFnName for SettingsVarType {
    fn write_setting_value_fn(&self) -> &'static str {
        match self {
            SettingsVarType::Bool => "WriteBoolSettingValue",
            SettingsVarType::Int {..} => "WriteIntSettingValue",
            SettingsVarType::Float {..} => "WriteFloatSettingValue",
            SettingsVarType::Enum { val_mapping, .. } => {
                if val_mapping.is_some() {
                    "WriteUnifiedEnumSettingValue"
                } else {
                    "WriteIntSettingValue"
                }
            }
        }
    }
}