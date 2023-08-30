// Code generated using Mod Settings Framework v0.5.0 by SpontanCombust & Aeltoth

class ModDifficultySettingsBase extends ISettingsMaster
{
	default modVersion = "1.1";

	public var general : ModDifficultySettingsBase_general;

	public /* override */ function Init() : void
	{
		general = new ModDifficultySettingsBase_general in this; general.Init(this);

		super.Init();
	}

	public /* override */ function ValidateSettings() : void
	{
		general.Validate();

		super.ValidateSettings();
	}

	public /* override */ function ReadSettings() : void
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		general.Read(config);

		super.ReadSettings();
	}

	public /* override */ function WriteSettings() : void
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		ValidateSettings();

		WriteBoolSettingValue(config, 'DMgeneral', 'DMenabled', general.enabled);
		WriteFloatSettingValue(config, 'DMgeneral', 'DMhealthMultip', general.healthMultip);
		WriteFloatSettingValue(config, 'DMgeneral', 'DMdmgMultip', general.dmgMultip);

		super.WriteSettings();
	}

	public /* override */ function ResetSettingsToDefault() : void
	{
		general.ResetToDefault(false);

		super.ResetSettingsToDefault();
	}

	public /* override */ function ShouldResetSettingsToDefaultOnInit() : bool
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

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

	public /* override */ function Validate() : void
	{
		healthMultip = ClampF(healthMultip, 0, 2);
		dmgMultip = ClampF(dmgMultip, 0, 2);

		super.Validate();
	}

	public /* override */ function Read(optional config: CInGameConfigWrapper) : void
	{
		if (!config)
			config = theGame.GetInGameConfigWrapper();

		enabled = m_parentMaster.ReadBoolSettingValue(config, 'DMgeneral', 'DMenabled');
		healthMultip = m_parentMaster.ReadFloatSettingValue(config, 'DMgeneral', 'DMhealthMultip');
		dmgMultip = m_parentMaster.ReadFloatSettingValue(config, 'DMgeneral', 'DMdmgMultip');

		Validate();

		super.Read(config);
	}
}

