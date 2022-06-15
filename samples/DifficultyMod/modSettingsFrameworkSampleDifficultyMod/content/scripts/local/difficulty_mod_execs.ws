exec function settings_difficulty_init()
{
    var game: CR4Game;

    game = theGame;
    if(!game.difficultySettings) {
        game.difficultySettings = new ModDifficultySettings in theGame;
        GetSettingsMasterRegistry().AddSettings(game.difficultySettings, 'DifficultySettings');
    }
}

exec function settings_difficulty_easy()
{
    theGame.difficultySettings.general.Reset(0);
}

exec function settings_difficulty_default()
{
    theGame.difficultySettings.general.ResetToDefault();
}

exec function settings_difficulty_hard()
{
    var game: CR4Game;

    game = theGame;
    game.difficultySettings.general.enabled = true;
    game.difficultySettings.general.healthMultip = 2.0;
    game.difficultySettings.general.dmgMultip = 2.0;

    game.difficultySettings.WriteSettings();
}

exec function settings_difficulty_toggle()
{
    var game: CR4Game;

    game = theGame;
    game.difficultySettings.general.enabled = !game.difficultySettings.general.enabled;
    game.difficultySettings.WriteSettings();
}

exec function settings_difficulty_read()
{
    theGame.difficultySettings.ReadSettings();
}

exec function settings_difficulty_write()
{
    theGame.difficultySettings.WriteSettings();
}

exec function settings_difficulty_log()
{
    LogChannel('DifficultyMod', 
        "Enabled: " + theGame.difficultySettings.general.enabled + ", " +
        "Health multiplier: " + theGame.difficultySettings.general.healthMultip + ", " +
        "Damage multiplier: " + theGame.difficultySettings.general.dmgMultip
    );
}

