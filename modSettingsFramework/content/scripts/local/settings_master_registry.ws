struct SSettingsMasterRegistryEntry
{
    var settingsMaster : ISettingsMaster;
    var id : name; 
}

enum ESettingsReadEvent
{
    ESettingsReadEvent_EnterMainMenu,
    ESettingsReadEvent_LoadSave,
    ESettingsReadEvent_ClosePauseMenu
}

class CSettingsMasterRegistry
{
    private var m_settingsEntries : array<SSettingsMasterRegistryEntry>;
    private var m_readListeners : array<ISettingsReadListener>;

    public function AddSettings(settingsMaster : ISettingsMaster, id : name, optional shouldDoReadNow : bool) : void
    {
        var settingsEntry : SSettingsMasterRegistryEntry;

        if(shouldDoReadNow)
        {
            settingsMaster.ReadSettings();
        }

        settingsEntry.settingsMaster = settingsMaster;
        settingsEntry.id = id;

        m_settingsEntries.PushBack(settingsEntry);
    }

    // Returns NULL if no setting master is found with that ID
    public function GetSettings(id : name) : ISettingsMaster
    {
        var i, size : int;

        size = m_settingsEntries.Size();
        for (i = 0; i < size; i += 1)
        {
            if(m_settingsEntries[i].id == id)
            {
                return m_settingsEntries[i].settingsMaster;
            }
        }

        return NULL;
    }

    public function RemoveSettings(id : name) : void
    {
        var i, size : int;

        size = m_settingsEntries.Size();
        for (i = 0; i < size; i += 1)
        {
            if(m_settingsEntries[i].id == id)
            {
                m_settingsEntries.Erase(i);
                return;
            }
        }
    }

    public function ReadAllSettings() : void
    {
        var i, size : int;

        size = m_settingsEntries.Size();
        for (i = 0; i < size; i += 1)
        {
            m_settingsEntries[i].ReadSettings();
        }

        size = m_readListeners.Size();
        for (i = 0; i < size; i += 1)
        {
            m_readListeners[i].OnReadSettings();
        }
    }

    public function AddReadListener(listener : ISettingsReadListener) : void
    {
        m_readListeners.PushBack(listener);
    }
}


function GetSettingsMasterRegistry() : CSettingsMasterRegistry
{
    if(theGame.m_settingsMasterRegistry == NULL)
    {
        theGame.m_settingsMasterRegistry = new CSettingsMasterRegistry in theGame;
    }

    return theGame.m_settingsMasterRegistry;
}
