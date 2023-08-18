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
		general.healthMultip = ClampF(general.healthMultip, 0, 2);
		general.dmgMultip = ClampF(general.dmgMultip, 0, 2);

		super.ValidateSettings();
	}

	public /* override */ function ReadSettings() : void
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		general.enabled = ReadBoolSettingValue(config, 'DMgeneral', 'enabled');
		general.healthMultip = ReadFloatSettingValue(config, 'DMgeneral', 'healthMultip');
		general.dmgMultip = ReadFloatSettingValue(config, 'DMgeneral', 'dmgMultip');

		ValidateSettings();

		super.ReadSettings();
	}

	public /* override */ function WriteSettings() : void
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		ValidateSettings();

		WriteBoolSettingValue(config, 'DMgeneral', 'enabled', general.enabled);
		WriteFloatSettingValue(config, 'DMgeneral', 'healthMultip', general.healthMultip);
		WriteFloatSettingValue(config, 'DMgeneral', 'dmgMultip', general.dmgMultip);

		super.WriteSettings();
	}

	public /* override */ function ResetSettingsToDefault() : void
	{
		general.ResetToDefault();
	}

	public /* override */ function ShouldResetSettingsToDefaultOnInit() : bool
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		return config.GetVarValue('DMgeneral','enabled') == "";
	}
}

class ModDifficultySettingsBase_general extends ISettingsGroup
{
	public var enabled : bool;
	public var healthMultip : float;
	public var dmgMultip : float;

	default id = 'DMgeneral';
	default defaultPresetIndex = 1;
}

