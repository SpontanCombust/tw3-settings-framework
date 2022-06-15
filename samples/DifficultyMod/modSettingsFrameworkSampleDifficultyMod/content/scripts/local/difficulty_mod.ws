class ModDifficultySettings extends ModDifficultySettingsBase
{
    public function Init() : void
    {
        super.Init();

        LogChannel('DifficultyMod', "Mod has been initialized");
    }   

    public function ReadSettings() : void
    {
        super.ReadSettings();

        LogChannel('DifficultyMod', "Mod settings have been read from config");
    }

    public function WriteSettings() : void
    {
        super.WriteSettings();

        LogChannel('DifficultyMod', "Mod has been written to config");
    }

    public function ShouldResetSettingsToDefaultOnInit() : bool
    {
        var v: float;

        v = StringToFloat(this.modVersion, 1.0);

        return super.ShouldResetSettingsToDefaultOnInit() || v > 1.0; 
    }

    public function ResetSettingValues(config: CInGameConfigWrapper, groupId: name, presetIndex: int) : void
    {
        super.ResetSettingValues(config, groupId, presetIndex);
        
        LogChannel('DifficultyMod', "Preset " + IntToString(presetIndex) + " has been applied" );
    }
}