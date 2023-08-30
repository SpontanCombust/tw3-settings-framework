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
		difficulties.Validate();
		monsters.Validate();

		super.ValidateSettings();
	}

	public /* override */ function ReadSettings() : void
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		difficulties.Read(config);
		monsters.Read(config);

		super.ReadSettings();
	}

	public /* override */ function WriteSettings() : void
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		difficulties.Write(false, config);
		monsters.Write(false, config);

		super.WriteSettings();
	}

	public /* override */ function ResetSettingsToDefault() : void
	{
		difficulties.ResetToDefault(false);
		monsters.ResetToDefault(false);

		super.ResetSettingsToDefault();
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

		return -1;
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

		return -1;
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
	public var noMansLand : MOTWDifficulty;
	public var skellige : MOTWDifficulty;
	public var kaerMorhen : MOTWDifficulty;
	public var toussaint : MOTWDifficulty;

	default id = 'MOTWdifficulties';
	default defaultPresetIndex = 0;

	public /* override */ function Validate() : void
	{
		noMansLand = (MOTWDifficulty)Clamp((int)noMansLand, 0, 2);
		skellige = (MOTWDifficulty)Clamp((int)skellige, 0, 2);
		kaerMorhen = (MOTWDifficulty)Clamp((int)kaerMorhen, 0, 2);
		toussaint = (MOTWDifficulty)Clamp((int)toussaint, 0, 2);

		super.Validate();
	}

	public /* override */ function Read(optional config: CInGameConfigWrapper) : void
	{
		if (!config)
			config = theGame.GetInGameConfigWrapper();

		noMansLand = (MOTWDifficulty)m_parentMaster.ReadIntSettingValue(config, 'MOTWdifficulties', 'MOTWnoMansLand');
		skellige = (MOTWDifficulty)m_parentMaster.ReadIntSettingValue(config, 'MOTWdifficulties', 'MOTWskellige');
		kaerMorhen = (MOTWDifficulty)m_parentMaster.ReadIntSettingValue(config, 'MOTWdifficulties', 'MOTWkaerMorhen');
		toussaint = (MOTWDifficulty)m_parentMaster.ReadIntSettingValue(config, 'MOTWdifficulties', 'MOTWtoussaint');

		Validate();

		super.Read(config);
	}

	public /* override */ function Write(shouldSave: bool, optional config: CInGameConfigWrapper) : void
	{
		if (!config)
			config = theGame.GetInGameConfigWrapper();

		Validate();

		m_parentMaster.WriteIntSettingValue(config, 'MOTWdifficulties', 'MOTWnoMansLand', (int)noMansLand);
		m_parentMaster.WriteIntSettingValue(config, 'MOTWdifficulties', 'MOTWskellige', (int)skellige);
		m_parentMaster.WriteIntSettingValue(config, 'MOTWdifficulties', 'MOTWkaerMorhen', (int)kaerMorhen);
		m_parentMaster.WriteIntSettingValue(config, 'MOTWdifficulties', 'MOTWtoussaint', (int)toussaint);

		super.Write(shouldSave, config);
	}
}

class MonsterOfTheWeekSettings_monsters extends ISettingsGroup
{
	public var noMansLand : MonsterOfTheWeekSettings_monster;
	public var skellige : MonsterOfTheWeekSettings_monster;
	public var kaerMorhen : MonsterOfTheWeekSettings_monster;
	public var toussaint : MonsterOfTheWeekSettings_monster;

	default id = 'MOTWmonsters';
	default defaultPresetIndex = 0;

	public /* override */ function Validate() : void
	{
		noMansLand = (MonsterOfTheWeekSettings_monster)m_parentMaster.EnumValueMappingValidateUnified('MOTWmonsters', 'MOTWnoMansLand', (int)noMansLand);
		skellige = (MonsterOfTheWeekSettings_monster)m_parentMaster.EnumValueMappingValidateUnified('MOTWmonsters', 'MOTWskellige', (int)skellige);
		kaerMorhen = (MonsterOfTheWeekSettings_monster)m_parentMaster.EnumValueMappingValidateUnified('MOTWmonsters', 'MOTWkaerMorhen', (int)kaerMorhen);
		toussaint = (MonsterOfTheWeekSettings_monster)m_parentMaster.EnumValueMappingValidateUnified('MOTWmonsters', 'MOTWtoussaint', (int)toussaint);

		super.Validate();
	}

	public /* override */ function Read(optional config: CInGameConfigWrapper) : void
	{
		if (!config)
			config = theGame.GetInGameConfigWrapper();

		noMansLand = (MonsterOfTheWeekSettings_monster)m_parentMaster.ReadUnifiedEnumSettingValue(config, 'MOTWmonsters', 'MOTWnoMansLand');
		skellige = (MonsterOfTheWeekSettings_monster)m_parentMaster.ReadUnifiedEnumSettingValue(config, 'MOTWmonsters', 'MOTWskellige');
		kaerMorhen = (MonsterOfTheWeekSettings_monster)m_parentMaster.ReadUnifiedEnumSettingValue(config, 'MOTWmonsters', 'MOTWkaerMorhen');
		toussaint = (MonsterOfTheWeekSettings_monster)m_parentMaster.ReadUnifiedEnumSettingValue(config, 'MOTWmonsters', 'MOTWtoussaint');

		Validate();

		super.Read(config);
	}

	public /* override */ function Write(shouldSave: bool, optional config: CInGameConfigWrapper) : void
	{
		if (!config)
			config = theGame.GetInGameConfigWrapper();

		Validate();

		m_parentMaster.WriteUnifiedEnumSettingValue(config, 'MOTWmonsters', 'MOTWnoMansLand', (int)noMansLand);
		m_parentMaster.WriteUnifiedEnumSettingValue(config, 'MOTWmonsters', 'MOTWskellige', (int)skellige);
		m_parentMaster.WriteUnifiedEnumSettingValue(config, 'MOTWmonsters', 'MOTWkaerMorhen', (int)kaerMorhen);
		m_parentMaster.WriteUnifiedEnumSettingValue(config, 'MOTWmonsters', 'MOTWtoussaint', (int)toussaint);

		super.Write(shouldSave, config);
	}
}

enum MOTWDifficulty
{
	MOTWDifficultyEasy = 0,
	MOTWDifficultyMedium = 1,
	MOTWDifficultyHard = 2,
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
