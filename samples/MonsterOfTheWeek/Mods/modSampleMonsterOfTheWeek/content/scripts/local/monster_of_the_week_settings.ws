// Code generated using Mod Settings Framework v0.5.0 by SpontanCombust & Aeltoth

class MonsterOfTheWeekSettings extends ISettingsMaster
{
	default modVersion = "1.0";

	public var difficulties : MonsterOfTheWeekSettings_difficulties;
	public var monsters : MonsterOfTheWeekSettings_monsters;

	public /* override */ function Init() : void
	{
		difficulties = new MonsterOfTheWeekSettings_difficulties in this; difficulties.Init(this);
		monsters = new MonsterOfTheWeekSettings_monsters in this; monsters.Init(this);

		super.Init();
	}

	public /* override */ function ValidateSettings() : void
	{
		difficulties.noMansLand = (MonsterOfTheWeekSettings_difficulty)Clamp((int)difficulties.noMansLand, 0, 2);
		difficulties.skellige = (MonsterOfTheWeekSettings_difficulty)Clamp((int)difficulties.skellige, 0, 2);
		difficulties.kaerMorhen = (MonsterOfTheWeekSettings_difficulty)Clamp((int)difficulties.kaerMorhen, 0, 2);
		difficulties.toussaint = (MonsterOfTheWeekSettings_difficulty)Clamp((int)difficulties.toussaint, 0, 2);

		monsters.noMansLand = (MonsterOfTheWeekSettings_monster)EnumValueMappingValidateUnified('MOTWmonsters', 'MOTWnoMansLand', (int)monsters.noMansLand);
		monsters.skellige = (MonsterOfTheWeekSettings_monster)EnumValueMappingValidateUnified('MOTWmonsters', 'MOTWskellige', (int)monsters.skellige);
		monsters.kaerMorhen = (MonsterOfTheWeekSettings_monster)EnumValueMappingValidateUnified('MOTWmonsters', 'MOTWkaerMorhen', (int)monsters.kaerMorhen);
		monsters.toussaint = (MonsterOfTheWeekSettings_monster)EnumValueMappingValidateUnified('MOTWmonsters', 'MOTWtoussaint', (int)monsters.toussaint);

		super.ValidateSettings();
	}

	public /* override */ function ReadSettings() : void
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		difficulties.noMansLand = (MonsterOfTheWeekSettings_difficulty)StringToInt(ReadSettingValue(config, 'MOTWdifficulties', 'MOTWnoMansLand'), 0);
		difficulties.skellige = (MonsterOfTheWeekSettings_difficulty)StringToInt(ReadSettingValue(config, 'MOTWdifficulties', 'MOTWskellige'), 0);
		difficulties.kaerMorhen = (MonsterOfTheWeekSettings_difficulty)StringToInt(ReadSettingValue(config, 'MOTWdifficulties', 'MOTWkaerMorhen'), 0);
		difficulties.toussaint = (MonsterOfTheWeekSettings_difficulty)StringToInt(ReadSettingValue(config, 'MOTWdifficulties', 'MOTWtoussaint'), 0);

		monsters.noMansLand = (MonsterOfTheWeekSettings_monster)EnumValueMappingConfigToUnified('MOTWmonsters', 'MOTWnoMansLand', StringToInt(ReadSettingValue(config, 'MOTWmonsters', 'MOTWnoMansLand'), 0));
		monsters.skellige = (MonsterOfTheWeekSettings_monster)EnumValueMappingConfigToUnified('MOTWmonsters', 'MOTWskellige', StringToInt(ReadSettingValue(config, 'MOTWmonsters', 'MOTWskellige'), 0));
		monsters.kaerMorhen = (MonsterOfTheWeekSettings_monster)EnumValueMappingConfigToUnified('MOTWmonsters', 'MOTWkaerMorhen', StringToInt(ReadSettingValue(config, 'MOTWmonsters', 'MOTWkaerMorhen'), 0));
		monsters.toussaint = (MonsterOfTheWeekSettings_monster)EnumValueMappingConfigToUnified('MOTWmonsters', 'MOTWtoussaint', StringToInt(ReadSettingValue(config, 'MOTWmonsters', 'MOTWtoussaint'), 0));

		this.ValidateSettings();
		super.ReadSettings();
	}

	public /* override */ function WriteSettings() : void
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		this.ValidateSettings();

		WriteSettingValue(config, 'MOTWdifficulties', 'MOTWnoMansLand', IntToString((int)difficulties.noMansLand));
		WriteSettingValue(config, 'MOTWdifficulties', 'MOTWskellige', IntToString((int)difficulties.skellige));
		WriteSettingValue(config, 'MOTWdifficulties', 'MOTWkaerMorhen', IntToString((int)difficulties.kaerMorhen));
		WriteSettingValue(config, 'MOTWdifficulties', 'MOTWtoussaint', IntToString((int)difficulties.toussaint));

		WriteSettingValue(config, 'MOTWmonsters', 'MOTWnoMansLand', IntToString(EnumValueMappingUnifiedToConfig('MOTWmonsters', 'MOTWnoMansLand', (int)monsters.noMansLand)));
		WriteSettingValue(config, 'MOTWmonsters', 'MOTWskellige', IntToString(EnumValueMappingUnifiedToConfig('MOTWmonsters', 'MOTWskellige', (int)monsters.skellige)));
		WriteSettingValue(config, 'MOTWmonsters', 'MOTWkaerMorhen', IntToString(EnumValueMappingUnifiedToConfig('MOTWmonsters', 'MOTWkaerMorhen', (int)monsters.kaerMorhen)));
		WriteSettingValue(config, 'MOTWmonsters', 'MOTWtoussaint', IntToString(EnumValueMappingUnifiedToConfig('MOTWmonsters', 'MOTWtoussaint', (int)monsters.toussaint)));

		super.WriteSettings();
	}

	public /* override */ function ResetSettingsToDefault() : void
	{
		difficulties.ResetToDefault();
		monsters.ResetToDefault();
	}

	public /* override */ function ShouldResetSettingsToDefaultOnInit() : bool
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		return config.GetVarValue('MOTWdifficulties','MOTWnoMansLand') == "";
	}

	public /* override */ function EnumValueMappingConfigToUnified(gId: name, vId: name, val: int) : int
	{
		switch(gId)
		{
		case 'MOTWmonsters':
			switch(vId)
			{
			case 'MOTWnoMansLand':
				switch(val)
				{
				case 0: return 0;
				case 1: return 1;
				case 2: return 2;
				case 3: return 3;
				case 4: return 4;
				case 5: return 5;
				case 6: return 6;
				}
			case 'MOTWskellige':
				switch(val)
				{
				case 0: return 7;
				case 1: return 8;
				case 2: return 1;
				case 3: return 2;
				case 4: return 3;
				case 5: return 4;
				case 6: return 9;
				case 7: return 10;
				case 8: return 11;
				}
			case 'MOTWkaerMorhen':
				switch(val)
				{
				case 0: return 8;
				case 1: return 2;
				case 2: return 1;
				case 3: return 3;
				}
			case 'MOTWtoussaint':
				switch(val)
				{
				case 0: return 12;
				case 1: return 13;
				case 2: return 14;
				case 3: return 15;
				case 4: return 16;
				case 5: return 17;
				case 6: return 18;
				}
			}
		}

		return val;
	}

	public /* override */ function EnumValueMappingUnifiedToConfig(gId: name, vId: name, val: int) : int
	{
		switch(gId)
		{
		case 'MOTWmonsters':
			switch(vId)
			{
			case 'MOTWnoMansLand':
				switch(val)
				{
				case 0: return 0;
				case 1: return 1;
				case 2: return 2;
				case 3: return 3;
				case 4: return 4;
				case 5: return 5;
				case 6: return 6;
				}
			case 'MOTWskellige':
				switch(val)
				{
				case 7: return 0;
				case 8: return 1;
				case 1: return 2;
				case 2: return 3;
				case 3: return 4;
				case 4: return 5;
				case 9: return 6;
				case 10: return 7;
				case 11: return 8;
				}
			case 'MOTWkaerMorhen':
				switch(val)
				{
				case 8: return 0;
				case 2: return 1;
				case 1: return 2;
				case 3: return 3;
				}
			case 'MOTWtoussaint':
				switch(val)
				{
				case 12: return 0;
				case 13: return 1;
				case 14: return 2;
				case 15: return 3;
				case 16: return 4;
				case 17: return 5;
				case 18: return 6;
				}
			}
		}

		return 0;
	}

	public /* override */ function EnumValueMappingValidateUnified(gId: name, vId: name, val: int) : int
	{
		switch(gId)
		{
		case 'MOTWmonsters':
			switch(vId)
			{
			case 'MOTWnoMansLand':
				switch(val)
				{
				case 0: 
				case 1: 
				case 2: 
				case 3: 
				case 4: 
				case 5: 
				case 6: 
					return val;
				default:
					return 0;
				}
			case 'MOTWskellige':
				switch(val)
				{
				case 7: 
				case 8: 
				case 1: 
				case 2: 
				case 3: 
				case 4: 
				case 9: 
				case 10: 
				case 11: 
					return val;
				default:
					return 7;
				}
			case 'MOTWkaerMorhen':
				switch(val)
				{
				case 8: 
				case 2: 
				case 1: 
				case 3: 
					return val;
				default:
					return 8;
				}
			case 'MOTWtoussaint':
				switch(val)
				{
				case 12: 
				case 13: 
				case 14: 
				case 15: 
				case 16: 
				case 17: 
				case 18: 
					return val;
				default:
					return 12;
				}
			}
		}

		return 0;
	}
}

class MonsterOfTheWeekSettings_difficulties extends ISettingsGroup
{
	public var noMansLand : MonsterOfTheWeekSettings_difficulty;
	public var skellige : MonsterOfTheWeekSettings_difficulty;
	public var kaerMorhen : MonsterOfTheWeekSettings_difficulty;
	public var toussaint : MonsterOfTheWeekSettings_difficulty;

	default id = 'MOTWdifficulties';
	default defaultPresetIndex = 0;
}

class MonsterOfTheWeekSettings_monsters extends ISettingsGroup
{
	public var noMansLand : MonsterOfTheWeekSettings_monster;
	public var skellige : MonsterOfTheWeekSettings_monster;
	public var kaerMorhen : MonsterOfTheWeekSettings_monster;
	public var toussaint : MonsterOfTheWeekSettings_monster;

	default id = 'MOTWmonsters';
	default defaultPresetIndex = 0;
}

enum MonsterOfTheWeekSettings_difficulty
{
	MonsterOfTheWeekSettings_difficulty_easy = 0,
	MonsterOfTheWeekSettings_difficulty_medium = 1,
	MonsterOfTheWeekSettings_difficulty_hard = 2,
}

enum MonsterOfTheWeekSettings_monster
{
	MonsterOfTheWeekSettings_monster_ghoul = 0,
	MonsterOfTheWeekSettings_monster_griffin = 1,
	MonsterOfTheWeekSettings_monster_wolf = 2,
	MonsterOfTheWeekSettings_monster_nekker = 3,
	MonsterOfTheWeekSettings_monster_leshen = 4,
	MonsterOfTheWeekSettings_monster_golem = 5,
	MonsterOfTheWeekSettings_monster_wraith = 6,
	MonsterOfTheWeekSettings_monster_siren = 7,
	MonsterOfTheWeekSettings_monster_bear = 8,
	MonsterOfTheWeekSettings_monster_cyclops = 9,
	MonsterOfTheWeekSettings_monster_werewolf = 10,
	MonsterOfTheWeekSettings_monster_harpy = 11,
	MonsterOfTheWeekSettings_monster_archespore = 12,
	MonsterOfTheWeekSettings_monster_panther = 13,
	MonsterOfTheWeekSettings_monster_slyzard = 14,
	MonsterOfTheWeekSettings_monster_centipede = 15,
	MonsterOfTheWeekSettings_monster_shaelmaar = 16,
	MonsterOfTheWeekSettings_monster_bruxa = 17,
	MonsterOfTheWeekSettings_monster_fleder = 18,
}


function GetMonsterOfTheWeekSettings() : MonsterOfTheWeekSettings
{
	var settings: MonsterOfTheWeekSettings;

	settings = (MonsterOfTheWeekSettings)GetSettingsMasterRegistry().GetSettings('MonsterOfTheWeekSettings');
	if(!settings)
	{
		settings = new MonsterOfTheWeekSettings in theGame;
		GetSettingsMasterRegistry().AddSettings(settings, 'MonsterOfTheWeekSettings');
	}

	return settings;
}
