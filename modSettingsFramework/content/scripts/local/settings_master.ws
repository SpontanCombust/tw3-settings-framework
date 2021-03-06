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




    // ==== Get/Set functions - to be potentially overriden by the developer if default is not enough ====

    // Fetches setting value from CInGameConfigWrapper
    public function ReadSettingValue(config: CInGameConfigWrapper, groupId: name, varId: name) : string
    {
        return config.GetVarValue(groupId, varId);
    }

    // Writes setting value into CInGameConfigWrapper
    public function WriteSettingValue(config: CInGameConfigWrapper, groupId: name, varId: name, value: string) : void
    {
        config.SetVarValue(groupId, varId, value);
    }

    // Applies a preset to a group in CInGameConfigWrapper
    public function ResetSettingValues(config: CInGameConfigWrapper, groupId: name, presetIndex: int) : void
    {
        config.ApplyGroupPreset(groupId, presetIndex);
    }




    // ====================== Utility functions ======================

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
}