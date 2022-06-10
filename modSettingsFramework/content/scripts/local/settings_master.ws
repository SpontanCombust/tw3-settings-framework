abstract class ISettingsMaster
{
    // Reads all settings from CInGameConfigWrapper using ReadSettingValue and sets class variables
    public function ReadSettings() : void {}
    // Using class variables and WriteSettingValue sets all settings in CInGameConfigWrapper and saves user configuration
    public function WriteSettings() : void {}

    // Invoked after ReadSettings
    public function OnReadSettings() : void {}
    // Invoked after WriteSettings
    public function OnWriteSettings() : void {}


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
}