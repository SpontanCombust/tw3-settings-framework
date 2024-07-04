// +++ game/r4Game.ws +++

@addField(CR4Game)
public var m_settingsMasterRegistry : CSettingsMasterRegistry;

// --- game/r4Game.ws ---


// +++ game/gui/commonMainMenuBase.ws +++

@wrapMethod(CR4CommonMainMenuBase)
function OnConfigUI() {
    wrappedMethod();

    GetSettingsMasterRegistry().ReadAllSettings();
}

// --- game/gui/commonMainMenuBase.ws ---


// +++ game/gui/main_menu/ingameMenu.ws +++

@wrapMethod(CR4IngameMenu)
function SaveChangedSettings() {
    var shouldUpdate: bool;

    shouldUpdate = this.hasChangedOption;

    wrappedMethod();

    if (shouldUpdate) {
        LogChannel('ModSettingsFramework', "Detected some changes.");
        GetSettingsMasterRegistry().ReadAllSettings();
    }
}

// --- game/gui/main_menu/ingameMenu.ws ---