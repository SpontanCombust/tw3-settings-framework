exec function monster_of_the_week()
{
    var settings: MonsterOfTheWeekSettings;
    var area: EAreaName;

    area = theGame.GetCommonMapManager().GetCurrentArea();
    settings = GetMonsterOfTheWeekSettings();

    switch(area) 
    {
    case AN_NMLandNovigrad:
    case AN_Velen:
        theGame.GetGuiManager().ShowNotification("Current MOTW in Velen and Novigrad: " + settings.monsters.noMansLand);
        break;
    case AN_Skellige_ArdSkellig:
        theGame.GetGuiManager().ShowNotification("Current MOTW in Skellige: " + settings.monsters.skellige);
        break;
    case AN_Kaer_Morhen:
        theGame.GetGuiManager().ShowNotification("Current MOTW in Kaer Morhen: " + settings.monsters.kaerMorhen);
        break;
    case (EAreaName)AN_Dlc_Bob:
        theGame.GetGuiManager().ShowNotification("Current MOTW in Toussaint: " + settings.monsters.toussaint);
        break;
    default:
        theGame.GetGuiManager().ShowNotification("Current area not supported");
    }
}

exec function monster_of_the_week_next_monster()
{
    var settings: MonsterOfTheWeekSettings;
    var i: int;

    settings = GetMonsterOfTheWeekSettings();

    // In this example all options' values are advanced by one
    // Unified enums don't work well in situations where you want to get the next value for an option,
    // as these (integer) values underneath for a specific option are not sequential.
    // Therefore config-unified conversions need to be used here.
    // This is not a very practical example. It is merely to show whether framework behaves properly.

    LogChannel('MOTW', "Before");
    LogChannel('MOTW', "NML: " + settings.monsters.noMansLand);
    LogChannel('MOTW', "Skellige: " + settings.monsters.skellige);
    LogChannel('MOTW', "Kaer Morhen: " + settings.monsters.kaerMorhen);
    LogChannel('MOTW', "Toussaint: " + settings.monsters.toussaint);

    i = settings.monsters.EnumValueMappingUnifiedToConfig('MOTWnoMansLand', (int)settings.monsters.noMansLand);
    settings.monsters.noMansLand = (MonsterOfTheWeekSettings_monster)settings.monsters.EnumValueMappingConfigToUnified('MOTWnoMansLand', i + 1);
    LogChannel('MOTW', "NML: " + i + " -> " + (i + 1));

    i = settings.monsters.EnumValueMappingUnifiedToConfig('MOTWskellige', (int)settings.monsters.skellige);
    settings.monsters.skellige = (MonsterOfTheWeekSettings_monster)settings.monsters.EnumValueMappingConfigToUnified('MOTWskellige', i + 1);
    LogChannel('MOTW', "Skellige: " + i + " -> " + (i + 1));

    i = settings.monsters.EnumValueMappingUnifiedToConfig('MOTWkaerMorhen', (int)settings.monsters.kaerMorhen);
    settings.monsters.kaerMorhen = (MonsterOfTheWeekSettings_monster)settings.monsters.EnumValueMappingConfigToUnified('MOTWkaerMorhen', i + 1);
    LogChannel('MOTW', "Kaer Morhen: " + i + " -> " + (i + 1));

    i = settings.monsters.EnumValueMappingUnifiedToConfig('MOTWtoussaint', (int)settings.monsters.toussaint);
    settings.monsters.toussaint = (MonsterOfTheWeekSettings_monster)settings.monsters.EnumValueMappingConfigToUnified('MOTWtoussaint', i + 1);
    LogChannel('MOTW', "Toussaint: " + i + " -> " + (i + 1));

    // validation happens before writing
    settings.monsters.WriteSettings(true);

    LogChannel('MOTW', "After");
    LogChannel('MOTW', "NML: " + settings.monsters.noMansLand);
    LogChannel('MOTW', "Skellige: " + settings.monsters.skellige);
    LogChannel('MOTW', "Kaer Morhen: " + settings.monsters.kaerMorhen);
    LogChannel('MOTW', "Toussaint: " + settings.monsters.toussaint);
}