// Code generated using Mod Settings Framework v0.5.0 by SpontanCombust & Aeltoth

class MyModSettings extends ISettingsMaster
{
	default modVersion = "1.23";

	public var tab1 : MyModSettings_tab1;
	public var tab2 : MyModSettings_tab2;
	public var tab3 : MyModSettings_tab3;

	public /* override */ function Init() : void
	{
		tab1 = new MyModSettings_tab1 in this; tab1.Init(this);
		tab2 = new MyModSettings_tab2 in this; tab2.Init(this);
		tab3 = new MyModSettings_tab3 in this; tab3.Init(this);

		super.Init();
	}

	public /* override */ function ValidateSettings() : void
	{
		tab1.Validate();
		tab2.Validate();
		tab3.Validate();

		super.ValidateSettings();
	}

	public /* override */ function ReadSettings() : void
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		tab1.option = (MyModSettings_opt)ReadIntSettingValue(config, 'MODtab1', 'MODoption');
		tab1.sliderFloat = ReadFloatSettingValue(config, 'MODtab1', 'MODslider1');
		tab1.sliderInt = ReadIntSettingValue(config, 'MODtab1', 'MODslider2');
		tab1.toggle = ReadBoolSettingValue(config, 'MODtab1', 'MODtoggle');
		tab1.version = ReadFloatSettingValue(config, 'MODtab1', 'MODversion');

		tab2.anotherSlider = ReadFloatSettingValue(config, 'MODtab2subtab1', 'anotherSlider');

		tab3.anotherToggle = ReadBoolSettingValue(config, 'MODtab2subtab2', 'anotherToggle');

		ValidateSettings();

		super.ReadSettings();
	}

	public /* override */ function WriteSettings() : void
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		ValidateSettings();

		WriteIntSettingValue(config, 'MODtab1', 'MODoption', (int)tab1.option);
		WriteFloatSettingValue(config, 'MODtab1', 'MODslider1', tab1.sliderFloat);
		WriteIntSettingValue(config, 'MODtab1', 'MODslider2', tab1.sliderInt);
		WriteBoolSettingValue(config, 'MODtab1', 'MODtoggle', tab1.toggle);
		WriteFloatSettingValue(config, 'MODtab1', 'MODversion', tab1.version);

		WriteFloatSettingValue(config, 'MODtab2subtab1', 'anotherSlider', tab2.anotherSlider);

		WriteBoolSettingValue(config, 'MODtab2subtab2', 'anotherToggle', tab3.anotherToggle);

		super.WriteSettings();
	}

	public /* override */ function ResetSettingsToDefault() : void
	{
		tab1.ResetToDefault(false);
		tab2.ResetToDefault(false);
		tab3.ResetToDefault(false);

		super.ResetSettingsToDefault();
	}

	public /* override */ function ShouldResetSettingsToDefaultOnInit() : bool
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		return config.GetVarValue('MODtab1','MODoption') == "";
	}
}

class MyModSettings_tab1 extends ISettingsGroup
{
	public var option : MyModSettings_opt;
	public var sliderFloat : float;
	public var sliderInt : int;
	public var toggle : bool;
	public var version : float;

	default id = 'MODtab1';
	default defaultPresetIndex = 1;

	public /* override */ function Validate() : void
	{
		option = (MyModSettings_opt)Clamp((int)option, 0, 2);
		sliderFloat = ClampF(sliderFloat, 0, 1);
		sliderInt = Clamp(sliderInt, 0, 100);
		version = ClampF(version, 0, 100);

		super.Validate();
	}
}

class MyModSettings_tab2 extends ISettingsGroup
{
	public var anotherSlider : float;

	default id = 'MODtab2subtab1';
	default defaultPresetIndex = 0;

	public /* override */ function Validate() : void
	{
		anotherSlider = ClampF(anotherSlider, -100, 100);

		super.Validate();
	}
}

class MyModSettings_tab3 extends ISettingsGroup
{
	public var anotherToggle : bool;

	default id = 'MODtab2subtab2';
	default defaultPresetIndex = 0;

	public /* override */ function Validate() : void
	{

		super.Validate();
	}
}

enum MyModSettings_opt
{
	MyModSettings_opt1 = 0,
	MyModSettings_opt2 = 1,
	MyModSettings_opt2 = 2,
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
