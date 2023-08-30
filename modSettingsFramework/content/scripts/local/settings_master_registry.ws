class CSettingsMasterRegistry
{
    public const var FRAMEWORK_VERSION : name;
    default FRAMEWORK_VERSION = '0.5';

    private var m_settingsEntries : array<ISettingsMaster>;

    public function AddSettings(settingsMaster : ISettingsMaster, id : name) : void
    {
        var i, size : int;

        size = m_settingsEntries.Size();
        for (i = 0; i < size; i += 1)
        {
            if (m_settingsEntries[i].id == id)
            {
                LogChannel('ModSettingsFramework', "Attempt was made to add settings master '" + id + "' which already exists in the registry");
                return;
            }
        }

        settingsMaster.id = id;
        settingsMaster.Init();

        m_settingsEntries.PushBack(settingsMaster);
        LogChannel('ModSettingsFramework', "Added settings master '" + id + "' to the registry");
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
                return m_settingsEntries[i];
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
                LogChannel('ModSettingsFramework', "Removed settings master '" + id + "' from the registry");
                return;
            }
        }
    }

    public function ReadAllSettings() : void
    {
        var i, size : int;

        LogChannel('ModSettingsFramework', "Reading all settings");
        
        size = m_settingsEntries.Size();
        for (i = 0; i < size; i += 1)
        {
            m_settingsEntries[i].ReadSettings();
        }
    }
}


function GetSettingsMasterRegistry() : CSettingsMasterRegistry
{
    var game : CR4Game;

    game = theGame;

    if(!game.m_settingsMasterRegistry)
    {
        LogChannel('ModSettingsFramework', "Initialising the registry");
        game.m_settingsMasterRegistry = new CSettingsMasterRegistry in theGame;
    }

    return game.m_settingsMasterRegistry;
}
