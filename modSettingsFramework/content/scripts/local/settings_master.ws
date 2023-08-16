abstract class ISettingsMaster
{
    public const var modVersion: string;



    // ====================== Functions to be defined in class generated by parser  ======================

    // Initializes the settings class members and more
    public function Init() : void 
    {
        if(ShouldResetSettingsToDefaultOnInit())
        {
            ResetSettingsToDefault();
        }

        // child class will initialize variables and modVersion beforehand
        ReadSettings();
    }

    // Corrects values to ranges specified in the xml
    public function ValidateSettings() : void
    {
        // generated child class will correct these values in this function
    }

    // Reads all settings from CInGameConfigWrapper using ReadSettingValue and sets class variables
    public function ReadSettings() : void 
    {
        // child class will fetch config var values here
    }

    // Using class variables and WriteSettingValue sets all settings in CInGameConfigWrapper and saves user configuration
    public function WriteSettings() : void 
    {
        // child class will send var values to config beforehand 
        theGame.SaveUserSettings();
    }

    // Apply a default preset to all groups if possible
    public function ResetSettingsToDefault() : void
    {
        // child class will call reset code on every group here
    }

    public function ShouldResetSettingsToDefaultOnInit() : bool
    {
        // child class will evaluate mod version var here
        return false;
    }

    // Returns integer value of the unified enum type for options var index in user config
    // If the config value is not valid for given option, should return -1
    public function EnumValueMappingConfigToUnified(groupId: name, varId: name, val: int) : int
    {
        return -1;
    }

    // Returns the options var index in user config for integer value of unified enum
    // If the unified value is not valid for given option, should return -1
    public function EnumValueMappingUnifiedToConfig(groupId: name, varId: name, val: int) : int
    {
        return -1;
    }

    // If integer value for given enum variable is correct returns said value
    // Otherwise returns the smallest valid value
    public function EnumValueMappingValidateUnified(groupId: name, varId: name, val: int) : int
    {
        return 0;
    }




    // ==== Get/Set functions - to be potentially overriden by the developer if default is not enough ====

    // Fetches setting value from CInGameConfigWrapper
    public function ReadSettingValue(config: CInGameConfigWrapper, gId: name, vId: name) : string
    {
        return config.GetVarValue(gId, vId);
    }

    // Writes setting value into CInGameConfigWrapper
    public function WriteSettingValue(config: CInGameConfigWrapper, gId: name, vId: name, value: string) : void
    {
        config.SetVarValue(gId, vId, value);
    }

    // Applies a preset to a group in CInGameConfigWrapper
    public function ResetSettingValues(config: CInGameConfigWrapper, gId: name, presetIndex: int) : void
    {
        config.ApplyGroupPreset(gId, presetIndex);
    }




    // ====================== Utility functions ======================

    // these bool conversion functions are here for sanity sake, 
    // because an implicit conversion from string to bool doesn't sit right with me
    protected function StringToBool(s: string) : bool
    {
        if(s == "false" || s == "" || !s) {
            return false;
        } else {
            return true;
        }
    }

    protected function BoolToString(b: bool) : string
    {
        if(b) {
            return "true";
        } else {
            return "false";
        }
    }


    protected function ReadIntSettingValue(config: CInGameConfigWrapper, gId: name, vId: name) : int
    {
        return StringToInt(ReadSettingValue(config, gId, vId), 0);
    }

    protected function ReadFloatSettingValue(config: CInGameConfigWrapper, gId: name, vId: name) : float
    {
        return StringToFloat(ReadSettingValue(config, gId, vId), 0.0);
    }

    protected function ReadBoolSettingValue(config: CInGameConfigWrapper, gId: name, vId: name) : bool
    {
        return StringToBool(ReadSettingValue(config, gId, vId));
    }

    protected function ReadUnifiedEnumSettingValue(config: CInGameConfigWrapper, gId: name, vId: name) : int
    {
        return EnumValueMappingConfigToUnified(gId, vId, ReadIntSettingValue(config, gId, vId));
    }


    protected function WriteIntSettingValue(config: CInGameConfigWrapper, gId: name, vId: name, value: int) : void
    {
        WriteSettingValue(config, gId, vId, IntToString(value));
    }

    protected function WriteFloatSettingValue(config: CInGameConfigWrapper, gId: name, vId: name, value: float) : void
    {
        WriteSettingValue(config, gId, vId, FloatToString(value));
    }

    protected function WriteBoolSettingValue(config: CInGameConfigWrapper, gId: name, vId: name, value: bool) : void
    {
        WriteSettingValue(config, gId, vId, BoolToString(value));
    }

    protected function WriteUnifiedEnumSettingValue(config: CInGameConfigWrapper, gId: name, vId: name, value: int) : void
    {
        WriteIntSettingValue(config, gId, vId, EnumValueMappingUnifiedToConfig(gId, vId, value));
    }
}