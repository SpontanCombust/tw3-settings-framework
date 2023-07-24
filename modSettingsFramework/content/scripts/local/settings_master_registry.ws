struct SSettingsMasterRegistryEntry
{
    var settingsMaster : ISettingsMaster;
    var id : name; 
}

class CSettingsMasterRegistry
{
    private var m_settingsEntries : array<SSettingsMasterRegistryEntry>;

    public function AddSettings(settingsMaster : ISettingsMaster, id : name) : void
    {
        var i, size : int;
        var settingsEntry : SSettingsMasterRegistryEntry;

        size = m_settingsEntries.Size();
        for (i = 0; i < size; i += 1)
        {
            if (m_settingsEntries[i].id == id)
            {
                return;
            }
        }

        settingsMaster.Init();

        settingsEntry.settingsMaster = settingsMaster;
        settingsEntry.id = id;
        //TODO log these actions
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
            m_settingsEntries[i].settingsMaster.ReadSettings();
        }
    }
}


function GetSettingsMasterRegistry() : CSettingsMasterRegistry
{
    var game : CR4Game;

    game = theGame;

    if(!game.m_settingsMasterRegistry)
    {
        game.m_settingsMasterRegistry = new CSettingsMasterRegistry in theGame;
    }

    return game.m_settingsMasterRegistry;
}
