abstract class ISettingsGroup
{
    protected var m_parentMaster : ISettingsMaster;

    // these are set using 'default' in the child class
    public const var id: name;
    public const var defaultPresetIndex: name;


    public function Init(parent_: ISettingsMaster) : void
    {
        m_parentMaster = parent_;
    }

    public function Reset(presetIndex: int) : void
    {
        var config: CInGameConfigWrapper;

        config = theGame.GetInGameConfigWrapper();
        
        m_parentMaster.ResetSettingValues(config, id, presetIndex);

        theGame.SaveUserSettings();
        m_parentMaster.OnWriteSettings();
        m_parentMaster.ReadSettings(false); // get preset values back from config
    }

    public function ResetToDefault() : void
    {
        Reset(defaultPresetIndex);
    }
}