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
    //TODO update class specification
    public function Validate(): void
    {
        // generated child class will correct these values in this function
    }

    public function Read(optional config: CInGameConfigWrapper) : void 
    {
        // child class will fetch config var values here
    }

    public function Write(shouldSave: bool, optional config: CInGameConfigWrapper) : void 
    {
        // child class will send var values to config beforehand 
        if (shouldSave)
            theGame.SaveUserSettings();
    }

    public function Reset(presetIndex: int, shouldSave: bool) : void
    {
        var config: CInGameConfigWrapper;

        config = theGame.GetInGameConfigWrapper();
        
        m_parentMaster.ResetSettingValues(config, id, presetIndex);
        Read(config); // get preset values back from config
        
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