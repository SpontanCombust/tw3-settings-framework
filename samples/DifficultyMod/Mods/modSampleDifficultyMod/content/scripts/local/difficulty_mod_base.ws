// Code generated using Mod Settings Framework v0.6.0 by SpontanCombust & Aeltoth

class ModDifficultySettingsBase extends ISettingsMaster
{
	default modVersion = "1.1";

	public var general : ModDifficultySettingsBase_general;

	protected /* override */ function Parser_Init() : void
	{
		general = new ModDifficultySettingsBase_general in this;
		general.Init(this);
		m_groups.PushBack(general);
	}

	protected /* override */ function Parser_ShouldResetSettingsToDefaultOnInit(config : CInGameConfigWrapper) : bool
	{
		return config.GetVarValue('DMgeneral','DMenabled') == "";
	}
}

class ModDifficultySettingsBase_general extends ISettingsGroup
{
	public var enabled : bool;
	public var healthMultip : float;
	public var dmgMultip : float;

	default id = 'DMgeneral';
	default defaultPresetIndex = 1;

	protected /* override */ function Parser_ValidateSettings() : void
	{
		healthMultip = ClampF(healthMultip, 0, 2);
		dmgMultip = ClampF(dmgMultip, 0, 2);
	}

	protected /* override */ function Parser_ReadSettings(config: CInGameConfigWrapper) : void
	{
		enabled = ReadBoolSettingValue(config, 'DMenabled');
		healthMultip = ReadFloatSettingValue(config, 'DMhealthMultip');
		dmgMultip = ReadFloatSettingValue(config, 'DMdmgMultip');
	}

	protected /* override */ function Parser_WriteSettings(config: CInGameConfigWrapper) : void
	{
		WriteBoolSettingValue(config, 'DMenabled', enabled);
		WriteFloatSettingValue(config, 'DMhealthMultip', healthMultip);
		WriteFloatSettingValue(config, 'DMdmgMultip', dmgMultip);
	}
}

