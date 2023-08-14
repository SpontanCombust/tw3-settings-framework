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

    // Unified enums don't work well in situations where you want to get the next value for an option,
    // as these (integer) values underneath for a specific option are not sequential.
    // Therefore config-unified conversions need to be used here.
    // This is not a very practical example. It is merely to show whether framework behaves properly.
    i = settings.EnumValueMappingUnifiedToConfig('MOTWmonsters', 'MOTWnoMansLand', (int)settings.monsters.noMansLand);
    settings.monsters.noMansLand = (MonsterOfTheWeekSettings_monster)settings.EnumValueMappingConfigToUnified('MOTWmonsters', 'MOTWnoMansLand', i + 1);
    LogChannel('MOTW', "NML: " + i + ", " + settings.monsters.noMansLand);

    i = settings.EnumValueMappingUnifiedToConfig('MOTWmonsters', 'MOTWskellige', (int)settings.monsters.skellige);
    settings.monsters.skellige = (MonsterOfTheWeekSettings_monster)settings.EnumValueMappingConfigToUnified('MOTWmonsters', 'MOTWskellige', i + 1);
    LogChannel('MOTW', "Skellige: " + i + ", " + settings.monsters.skellige);

    i = settings.EnumValueMappingUnifiedToConfig('MOTWmonsters', 'MOTWkaerMorhen', (int)settings.monsters.kaerMorhen);
    settings.monsters.kaerMorhen = (MonsterOfTheWeekSettings_monster)settings.EnumValueMappingConfigToUnified('MOTWmonsters', 'MOTWkaerMorhen', i + 1);
    LogChannel('MOTW', "Kaer Morhen: " + i + ", " + settings.monsters.kaerMorhen);

    i = settings.EnumValueMappingUnifiedToConfig('MOTWmonsters', 'MOTWtoussaint', (int)settings.monsters.toussaint);
    settings.monsters.toussaint = (MonsterOfTheWeekSettings_monster)settings.EnumValueMappingConfigToUnified('MOTWmonsters', 'MOTWtoussaint', i + 1);
    LogChannel('MOTW', "Toussaint: " + i + ", " + settings.monsters.toussaint);

    settings.WriteSettings();
}