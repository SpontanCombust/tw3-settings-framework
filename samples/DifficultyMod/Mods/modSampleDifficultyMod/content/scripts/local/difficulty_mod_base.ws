// Code generated using Mod Settings Framework v0.5.0 by SpontanCombust & Aeltoth

class ModDifficultySettingsBase extends ISettingsMaster
{
	default modVersion = "1.1";

	public var general : ModDifficultySettingsBase_general;

	protected /* override */ function Parser_Init() : void
	{
		general = new ModDifficultySettingsBase_general in this; general.Init(this);
	}

	protected /* override */ function Parser_ValidateSettings() : void
	{
		general.Validate();
	}

	protected /* override */ function Parser_ReadSettings(config : CInGameConfigWrapper) : void
	{
		general.Read(config);
	}

	protected /* override */ function Parser_WriteSettings(config : CInGameConfigWrapper) : void
	{
		general.Write(false, config);
	}

	protected /* override */ function Parser_ResetSettingsToDefault(config : CInGameConfigWrapper) : void
	{
		general.ResetToDefault(false, config);
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

	protected /* override */ function Parser_Validate() : void
	{
		healthMultip = ClampF(healthMultip, 0, 2);
		dmgMultip = ClampF(dmgMultip, 0, 2);
	}

	public /* override */ function Parser_Read(config: CInGameConfigWrapper) : void
	{
		enabled = m_parentMaster.ReadBoolSettingValue(config, 'DMgeneral', 'DMenabled');
		healthMultip = m_parentMaster.ReadFloatSettingValue(config, 'DMgeneral', 'DMhealthMultip');
		dmgMultip = m_parentMaster.ReadFloatSettingValue(config, 'DMgeneral', 'DMdmgMultip');
	}

	protected /* override */ function Parser_Write(config: CInGameConfigWrapper) : void
	{
		m_parentMaster.WriteBoolSettingValue(config, 'DMgeneral', 'DMenabled', enabled);
		m_parentMaster.WriteFloatSettingValue(config, 'DMgeneral', 'DMhealthMultip', healthMultip);
		m_parentMaster.WriteFloatSettingValue(config, 'DMgeneral', 'DMdmgMultip', dmgMultip);
	}
}

