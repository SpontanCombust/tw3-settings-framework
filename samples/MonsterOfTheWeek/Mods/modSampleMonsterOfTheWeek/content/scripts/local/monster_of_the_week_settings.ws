// Code generated using Mod Settings Framework v0.5.0 by SpontanCombust & Aeltoth

class MonsterOfTheWeekSettings extends ISettingsMaster
{
	default modVersion = "1.0";

	public var difficulty : MonsterOfTheWeekSettings_difficulty;
	public var monsters : MonsterOfTheWeekSettings_monsters;

	public /* override */ function Init() : void
	{
		difficulty = new MonsterOfTheWeekSettings_difficulty in this; difficulty.Init(this);
		monsters = new MonsterOfTheWeekSettings_monsters in this; monsters.Init(this);

		super.Init();
	}

	public /* override */ function ValidateSettings() : void
	{
		difficulty.noMansLand = (MonsterOfTheWeekSettings_difficulty)Clamp((int)difficulty.noMansLand, 0, 2);
		difficulty.skellige = (MonsterOfTheWeekSettings_difficulty)Clamp((int)difficulty.skellige, 0, 2);
		difficulty.kaerMorhen = (MonsterOfTheWeekSettings_difficulty)Clamp((int)difficulty.kaerMorhen, 0, 2);
		difficulty.toussaint = (MonsterOfTheWeekSettings_difficulty)Clamp((int)difficulty.toussaint, 0, 2);

		monsters.noMansLand = (MonsterOfTheWeekSettings_monster)Clamp((int)monsters.noMansLand, 0, 6);
		monsters.skellige = (MonsterOfTheWeekSettings_monster)Clamp((int)monsters.skellige, 0, 8);
		monsters.kaerMorhen = (MonsterOfTheWeekSettings_monster)Clamp((int)monsters.kaerMorhen, 0, 3);
		monsters.toussaint = (MonsterOfTheWeekSettings_monster)Clamp((int)monsters.toussaint, 0, 6);

		super.ValidateSettings();
	}

	public /* override */ function ReadSettings() : void
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		difficulty.noMansLand = (MonsterOfTheWeekSettings_difficulty)StringToInt(ReadSettingValue(config, 'MOTWdifficulty', 'MOTWnoMansLand'), 0);
		difficulty.skellige = (MonsterOfTheWeekSettings_difficulty)StringToInt(ReadSettingValue(config, 'MOTWdifficulty', 'MOTWskellige'), 0);
		difficulty.kaerMorhen = (MonsterOfTheWeekSettings_difficulty)StringToInt(ReadSettingValue(config, 'MOTWdifficulty', 'MOTWkaerMorhen'), 0);
		difficulty.toussaint = (MonsterOfTheWeekSettings_difficulty)StringToInt(ReadSettingValue(config, 'MOTWdifficulty', 'MOTWtoussaint'), 0);

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

		WriteSettingValue(config, 'MOTWdifficulty', 'MOTWnoMansLand', IntToString((int)difficulty.noMansLand));
		WriteSettingValue(config, 'MOTWdifficulty', 'MOTWskellige', IntToString((int)difficulty.skellige));
		WriteSettingValue(config, 'MOTWdifficulty', 'MOTWkaerMorhen', IntToString((int)difficulty.kaerMorhen));
		WriteSettingValue(config, 'MOTWdifficulty', 'MOTWtoussaint', IntToString((int)difficulty.toussaint));

		WriteSettingValue(config, 'MOTWmonsters', 'MOTWnoMansLand', IntToString(EnumValueMappingUnifiedToConfig('MOTWmonsters', 'MOTWnoMansLand', (int)monsters.noMansLand)));
		WriteSettingValue(config, 'MOTWmonsters', 'MOTWskellige', IntToString(EnumValueMappingUnifiedToConfig('MOTWmonsters', 'MOTWskellige', (int)monsters.skellige)));
		WriteSettingValue(config, 'MOTWmonsters', 'MOTWkaerMorhen', IntToString(EnumValueMappingUnifiedToConfig('MOTWmonsters', 'MOTWkaerMorhen', (int)monsters.kaerMorhen)));
		WriteSettingValue(config, 'MOTWmonsters', 'MOTWtoussaint', IntToString(EnumValueMappingUnifiedToConfig('MOTWmonsters', 'MOTWtoussaint', (int)monsters.toussaint)));

		super.WriteSettings();
	}

	public /* override */ function ResetSettingsToDefault() : void
	{
		difficulty.ResetToDefault();
		monsters.ResetToDefault();
	}

	public /* override */ function ShouldResetSettingsToDefaultOnInit() : bool
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		return config.GetVarValue('MOTWdifficulty','MOTWnoMansLand') == "";
	}

	protected /* override */ function EnumValueMappingConfigToUnified(groupId: name, varId: name, val: int) : int
	{
		switch(groupId)
		{
		case 'MOTWmonsters':
			switch(varId)
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

	protected /* override */ function EnumValueMappingUnifiedToConfig(groupId: name, varId: name, val: int) : int
	{
		switch(groupId)
		{
		case 'MOTWmonsters':
			switch(varId)
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
}

class MonsterOfTheWeekSettings_difficulty extends ISettingsGroup
{
	public var noMansLand : MonsterOfTheWeekSettings_difficulty;
	public var skellige : MonsterOfTheWeekSettings_difficulty;
	public var kaerMorhen : MonsterOfTheWeekSettings_difficulty;
	public var toussaint : MonsterOfTheWeekSettings_difficulty;

	default id = 'MOTWdifficulty';
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

enum MonsterOfTheWeekSettings_difficulty
{
	MonsterOfTheWeekSettings_difficulty_easy = 0,
	MonsterOfTheWeekSettings_difficulty_medium = 1,
	MonsterOfTheWeekSettings_difficulty_hard = 2,
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
