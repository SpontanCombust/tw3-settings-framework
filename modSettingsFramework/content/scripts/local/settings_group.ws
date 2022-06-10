abstract class ISettingsGroup
{
    protected var m_parentMaster : ISettingsMaster;

    public var m_id: name;


    public function Init(parent_: ISettingsMaster, id : name) : void
    {
        m_parentMaster = parent_;
        m_id = id;
    }

    public function Reset(presetIndex: int)
    {
        var config: CInGameConfigWrapper;

        config = theGame.GetInGameConfigWrapper();
        
        m_parentMaster.ResetSettingValues(config, m_id, presetIndex);

        theGame.SaveUserSettings();
        m_parentMaster.OnWriteSettings();
        m_parentMaster.ReadSettings(false); // get preset values back from config
    }
}