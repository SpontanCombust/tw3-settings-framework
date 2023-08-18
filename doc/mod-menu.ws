// Code generated using Mod Settings Framework v0.5.0 by SpontanCombust & Aeltoth

class MyModSettings extends ISettingsMaster
{
	default modVersion = "1.23";

	public var tab1 : MyModSettings_tab1;
	public var tab2subtab1 : MyModSettings_tab2subtab1;
	public var tab2subtab2 : MyModSettings_tab2subtab2;

	public /* override */ function Init() : void
	{
		tab1 = new MyModSettings_tab1 in this; tab1.Init(this);
		tab2subtab1 = new MyModSettings_tab2subtab1 in this; tab2subtab1.Init(this);
		tab2subtab2 = new MyModSettings_tab2subtab2 in this; tab2subtab2.Init(this);

		super.Init();
	}

	public /* override */ function ValidateSettings() : void
	{
		tab1.option = (MyModSettings_mod_opt)Clamp((int)tab1.option, 0, 2);
		tab1.sliderFloat = ClampF(tab1.sliderFloat, 0, 1);
		tab1.sliderInt = Clamp(tab1.sliderInt, 0, 100);
		tab1.version = ClampF(tab1.version, 0, 100);

		tab2subtab1.anotherSlider = ClampF(tab2subtab1.anotherSlider, -100, 100);

		super.ValidateSettings();
	}

	public /* override */ function ReadSettings() : void
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		tab1.option = (MyModSettings_mod_opt)ReadIntSettingValue(config, 'MODtab1', 'option');
		tab1.sliderFloat = ReadFloatSettingValue(config, 'MODtab1', 'sliderFloat');
		tab1.sliderInt = ReadIntSettingValue(config, 'MODtab1', 'sliderInt');
		tab1.toggle = ReadBoolSettingValue(config, 'MODtab1', 'toggle');
		tab1.version = ReadFloatSettingValue(config, 'MODtab1', 'version');

		tab2subtab1.anotherSlider = ReadFloatSettingValue(config, 'MODtab2subtab1', 'anotherSlider');

		tab2subtab2.anotherToggle = ReadBoolSettingValue(config, 'MODtab2subtab2', 'anotherToggle');

		ValidateSettings();

		super.ReadSettings();
	}

	public /* override */ function WriteSettings() : void
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		ValidateSettings();

		WriteIntSettingValue(config, 'MODtab1', 'option', (int)tab1.option);
		WriteFloatSettingValue(config, 'MODtab1', 'sliderFloat', tab1.sliderFloat);
		WriteIntSettingValue(config, 'MODtab1', 'sliderInt', tab1.sliderInt);
		WriteBoolSettingValue(config, 'MODtab1', 'toggle', tab1.toggle);
		WriteFloatSettingValue(config, 'MODtab1', 'version', tab1.version);

		WriteFloatSettingValue(config, 'MODtab2subtab1', 'anotherSlider', tab2subtab1.anotherSlider);

		WriteBoolSettingValue(config, 'MODtab2subtab2', 'anotherToggle', tab2subtab2.anotherToggle);

		super.WriteSettings();
	}

	public /* override */ function ResetSettingsToDefault() : void
	{
		tab1.ResetToDefault();
		tab2subtab1.ResetToDefault();
		tab2subtab2.ResetToDefault();

		super.ResetSettingsToDefault();
	}

	public /* override */ function ShouldResetSettingsToDefaultOnInit() : bool
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		return config.GetVarValue('MODtab1','option') == "";
	}
}

class MyModSettings_tab1 extends ISettingsGroup
{
	public var option : MyModSettings_mod_opt;
	public var sliderFloat : float;
	public var sliderInt : int;
	public var toggle : bool;
	public var version : float;

	default id = 'MODtab1';
	default defaultPresetIndex = 1;
}

class MyModSettings_tab2subtab1 extends ISettingsGroup
{
	public var anotherSlider : float;

	default id = 'MODtab2subtab1';
	default defaultPresetIndex = 0;
}

class MyModSettings_tab2subtab2 extends ISettingsGroup
{
	public var anotherToggle : bool;

	default id = 'MODtab2subtab2';
	default defaultPresetIndex = 0;
}

enum MyModSettings_mod_opt
{
	MyModSettings_mod_opt1 = 0,
	MyModSettings_mod_opt2 = 1,
	MyModSettings_mod_opt2 = 2,
}


function GetMyModSettings() : MyModSettings
{
	var settings: MyModSettings;

	settings = (MyModSettings)GetSettingsMasterRegistry().GetSettings('MyModSettings');
	if(!settings)
	{
		settings = new MyModSettings in theGame;
		GetSettingsMasterRegistry().AddSettings(settings, 'MyModSettings');
	}

	return settings;
}
