exec function settings_difficulty_easy()
{
    GetModDifficultySettings().general.Reset(0, true);
}

exec function settings_difficulty_default()
{
    GetModDifficultySettings().general.ResetToDefault(true);
}

exec function settings_difficulty_hard()
{
    var settings : ModDifficultySettings;
    settings = GetModDifficultySettings();

    settings.general.enabled = true;
    settings.general.healthMultip = 2.0;
    settings.general.dmgMultip = 2.0;

    settings.WriteSettings();
}

exec function settings_difficulty_toggle()
{
    var settings : ModDifficultySettings;
    settings = GetModDifficultySettings();

    settings.general.enabled = !settings.general.enabled;
    settings.WriteSettings();
}

exec function settings_difficulty_read()
{
    GetModDifficultySettings().ReadSettings();
}

exec function settings_difficulty_write()
{
    GetModDifficultySettings().WriteSettings();
}

exec function settings_difficulty_log()
{
    var settings : ModDifficultySettings;
    settings = GetModDifficultySettings();

    LogChannel('DifficultyMod', 
        "Enabled: " + settings.general.enabled + ", " +
        "Health multiplier: " + settings.general.healthMultip + ", " +
        "Damage multiplier: " + settings.general.dmgMultip
    );
}

