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

    public function Reset(presetIndex: int) : void
    {
        var config: CInGameConfigWrapper;

        config = theGame.GetInGameConfigWrapper();
        
        m_parentMaster.ResetSettingValues(config, id, presetIndex);
        //TODO add parameter that says whether this should be done (if all groups are reset it is needlessly done multiple times)
        theGame.SaveUserSettings();
        m_parentMaster.ReadSettings(); // get preset values back from config
    }

    public function ResetToDefault() : void
    {
        Reset(defaultPresetIndex);
    }
}