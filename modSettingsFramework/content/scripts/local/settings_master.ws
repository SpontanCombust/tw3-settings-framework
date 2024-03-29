abstract class ISettingsMaster
{
    public const var modVersion: string;
    public var id: name;

    protected var m_groups: array<ISettingsGroup>;


    // ============================ Main functions, can be overriden  ==============================

    // Initializes the settings class members and more
    public function Init() : void 
    {
        var resetToDefault : bool;

        LogChannel('ModSettingsFramework', "Initialising settings master '" + id + "'");
        Parser_Init();

        resetToDefault = ShouldResetSettingsToDefaultOnInit();
        if (resetToDefault)
        {
            ResetSettingsToDefault();  
        }

        ReadSettings();

        if (resetToDefault)
        {
            // Resetting settings implies that there are presets to begin with
            // If for some reason there are none, nothing new will be written to user.config
            // If a mod was initialised for the first time then it will still not exist in user.config after closing the game
            // This can be midigated by forcing settings to be written for this mod
            // After calling ReadSettings() everything should have valid values, so writing to config should be more than safe
            // This is also done only once per session, so it shouldn't be an issue performance wise
            WriteSettings();
        }
    }

    // Corrects values to ranges specified in the xml
    public function ValidateSettings() : void
    {
        var i, size: int;
        var group: ISettingsGroup;

        LogChannel('ModSettingsFramework', "Validating settings for master '" + id + "'");
        size = m_groups.Size();
        for(i = 0; i < size; i += 1)
        {
            group = m_groups[i];
            group.ValidateSettings();
        }
    }

    // Reads all settings from CInGameConfigWrapper using ReadSettingValue and sets class variables
    public function ReadSettings() : void 
    {
        var i, size: int;
        var group: ISettingsGroup;
        var config: CInGameConfigWrapper;

        config = theGame.GetInGameConfigWrapper();

        LogChannel('ModSettingsFramework', "Reading settings for master '" + id + "'");
        size = m_groups.Size();
        for(i = 0; i < size; i += 1)
        {
            group = m_groups[i];
            group.ReadSettings(config);
        }
    }

    // Using class variables and WriteSettingValue sets all settings in CInGameConfigWrapper and saves user configuration
    public function WriteSettings() : void 
    {
        var i, size: int;
        var group: ISettingsGroup;
        var config: CInGameConfigWrapper;

        config = theGame.GetInGameConfigWrapper();

        LogChannel('ModSettingsFramework', "Writing settings for master '" + id + "'");
        size = m_groups.Size();
        for(i = 0; i < size; i += 1)
        {
            group = m_groups[i];
            group.WriteSettings(false, config);
        }

        theGame.SaveUserSettings();
    }

    // Apply a default preset to all groups if possible
    public function ResetSettingsToDefault() : void
    {
        var i, size: int;
        var group: ISettingsGroup;
        var config: CInGameConfigWrapper;

        config = theGame.GetInGameConfigWrapper();

        LogChannel('ModSettingsFramework', "Resetting settings to default for master '" + id + "'");
        size = m_groups.Size();
        for(i = 0; i < size; i += 1)
        {
            group = m_groups[i];
            group.ResetSettingsToDefault(false, config);
        }

        theGame.SaveUserSettings();
    }

    // Checks whether this mod's settings have been saved onto disk before
    public function ShouldResetSettingsToDefaultOnInit() : bool
    {
        var config: CInGameConfigWrapper;
        config = theGame.GetInGameConfigWrapper();

        return Parser_ShouldResetSettingsToDefaultOnInit(config);
    }




    // ==== Get/Set functions - to be potentially overriden by the developer if default is not enough ====

    // Fetches setting value from CInGameConfigWrapper
    // Master provides this function for group classes
    public function ReadSettingValue(config: CInGameConfigWrapper, gId: name, vId: name) : string
    {
        return config.GetVarValue(gId, vId);
    }

    // Writes setting value into CInGameConfigWrapper
    // Master provides this function for group classes
    public function WriteSettingValue(config: CInGameConfigWrapper, gId: name, vId: name, value: string) : void
    {
        config.SetVarValue(gId, vId, value);
    }

    // Applies a preset to a group in CInGameConfigWrapper
    // Master provides this function for group classes
    public function ResetSettingValues(config: CInGameConfigWrapper, gId: name, presetIndex: int) : void
    {
        config.ApplyGroupPreset(gId, presetIndex);
    }



    // ====================== Functions to be defined in class generated by parser  ======================

    protected function Parser_Init() : void {}
    protected function Parser_ShouldResetSettingsToDefaultOnInit(config: CInGameConfigWrapper) : bool { return false; }
}