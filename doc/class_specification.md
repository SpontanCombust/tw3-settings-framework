# Class specification

## [`CSettingsMasterRegistry`](../modSettingsFramework/content/scripts/local/settings_master_registry.ws)
Class that contains all settings master objects and updates them when necessary.

### Public member constants
- `FRAMEWORK_VERSION: name` - version of the framework 

### Public member functions
- `AddSettings(settingsMaster : ISettingsMaster, id : name): void` - adds a settings master object to the registry and initialises it. The 'id' parameter is used to identify said object later.
- `GetSettings(id : name) : ISettingsMaster` - returns stored settings master object or NULL if it can't find any with this id.
- `RemoveSettings(id : name) : void` - removes settings master object from the registry.
- `ReadAllSettings() : void` - reads settings values from user config into stored settings master objects.


## [`ISettingsMaster`](../modSettingsFramework/content/scripts/local/settings_master.ws)
Abstract base class for the settings class generated by the parser. The child class generated by the parser stores instances of classes extending ISettingsGroup. The class generated by the parser can be extended further to provide custom reactionary behaviour for its methods.

### Public member constants
- `modVersion: string` - arbitrary version string that can be passed during parsing.
- `id: name` - arbitrary identifier of the settings master object. Used by the registry to differentiate between them.

### Public member functions
- `Init() : void` - initializes settings groups, reads data from config and optionally resets this data to default if your mod has just been installed.
- `ReadSettings() : void` - calls Read() on all group objects.
- `WriteSettings() : void` - calls Write() on all group objects and saves data to disk.
- `ValidateSettings(): void` - calls Validate() on all group objects.
- `ResetSettingsToDefault() : void` - applies default presets to all groups and saves data to disk.
- `ShouldResetSettingsToDefaultOnInit() : bool` - a condition telling the class whether it should use ResetSettingsToDefault on Init. By default it checks whether GetVarValue() for a random (in reality the first one) var from the xml returns "".
- `ReadSettingValue(config: CInGameConfigWrapper, groupId: name, varId: name) : string` - method used by ReadSettings to retrieve values from game config. By default it used CInGameConfigWrapper directly.
- `WriteSettingValue(config: CInGameConfigWrapper, groupId: name, varId: name, value: string) : void` - method used by WriteSettings to write values into game config. By default it used CInGameConfigWrapper directly.
- `ResetSettingValues(config: CInGameConfigWrapper, groupId: name, presetIndex: int) : void` - method used by ISettingsGroup's Reset method to apply a preset. By default it used CInGameConfigWrapper directly.


## [ISettingsGroup](../modSettingsFramework/content/scripts/local/settings_group.ws)
Abstract base class which is an analogue to settings groups in XML. Child classes generated by the parser store settings variables.

### Public member constants
- `id: name` - value of the id attribute for the group in the XML.
- `defaultPresetIndex: int` - value of the default preset used by the given group. By default it's 0. To see how to instruct the parser deduce default presets refer to [XML specification](./xml_specification.md).

### Public member functions
- `Init(parent_: ISettingsMaster) : void` - used internally.
- `ValidateSettings() : void` - checks all fields and corrects their values so they adhere to the limits set in the XML, e.g. if the value of a variable corresponding to a slider config is within the correct range.
- `ReadSettings(optional config: CInGameConfigWrapper) : void` - reads the data from game's configuration.
- `WriteSettings(shouldSave: bool, optional config: CInGameConfigWrapper) : void` - writes the data into game's configuration. If 'shouldSave' is true saved the data to disk.  
- `ResetSettings(presetIndex: int, shouldSave: bool, optional config: CInGameConfigWrapper) : void` - applies a preset with given id. Automatically updates settings class to reflect this. If 'shouldSave' is true saved the data to disk.
- `ResetSettingsToDefault() : void` - calls Reset() with defaultPresetIndex.
- `EnumValueMappingConfigToUnified(vId: name, val: int) : int` - returns integer value of the unified enum type for options var index in user config. If the config value is not valid for given option, should return -1. More about unified enums in [details](./details.md).
- `EnumValueMappingUnifiedToConfig(vId: name, val: int) : int` - returns the options var index in user config for integer value of unified enum. If the unified value is not valid for given option, should return -1. More about unified enums in [details](./details.md).
- `EnumValueMappingValidateUnified(vId: name, val: int) : int` - If integer value for given enum variable is correct returns said value. Otherwise returns the smallest valid value. More about unified enums in [details](./details.md).
