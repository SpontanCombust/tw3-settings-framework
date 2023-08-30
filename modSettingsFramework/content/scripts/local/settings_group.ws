abstract class ISettingsGroup
{
    protected var m_parentMaster : ISettingsMaster;

    // these are set using 'default' in the child class
    public const var id: name;
    public const var defaultPresetIndex: int;


    public function Init(parent_: ISettingsMaster) : void
    {
        m_parentMaster = parent_;
    }

    public function Reset(presetIndex: int, shouldSave: bool) : void
    {
        var config: CInGameConfigWrapper;

        config = theGame.GetInGameConfigWrapper();
        
        m_parentMaster.ResetSettingValues(config, id, presetIndex);
        m_parentMaster.ReadSettings(); // get preset values back from config
        
        if (shouldSave)
        {
            theGame.SaveUserSettings();
        }
    }

    public function ResetToDefault(shouldSave: bool) : void
    {
        Reset(defaultPresetIndex, shouldSave);
    }
}