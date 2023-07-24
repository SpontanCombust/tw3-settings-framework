// Code generated using Mod Settings Framework v0.4.0 by SpontanCombust & Aeltoth

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

		general.enabled = StringToBool(ReadSettingValue(config, 'DMgeneral', 'DMenabled'));
		general.healthMultip = StringToFloat(ReadSettingValue(config, 'DMgeneral', 'DMhealthMultip'), 0.0);
		general.dmgMultip = StringToFloat(ReadSettingValue(config, 'DMgeneral', 'DMdmgMultip'), 0.0);

		this.ValidateSettings();
		super.ReadSettings();
	}

	public /* override */ function WriteSettings() : void
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		this.ValidateSettings();

		WriteSettingValue(config, 'DMgeneral', 'DMenabled', BoolToString(general.enabled));
		WriteSettingValue(config, 'DMgeneral', 'DMhealthMultip', FloatToString(general.healthMultip));
		WriteSettingValue(config, 'DMgeneral', 'DMdmgMultip', FloatToString(general.dmgMultip));

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
}

