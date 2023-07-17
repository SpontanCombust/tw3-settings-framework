// Code generated using Mod Settings Framework v0.3.0 by SpontanCombust & Aeltoth

class MyModSettings extends ISettingsMaster
{
	default modVersion = "1.23";

	public var tab1 : MyModSettings_tab1;
	public var tab2subtab1 : MyModSettings_tab2subtab1;
	public var tab2subtab2 : MyModSettings_tab2subtab2;

	public function Init() : void
	{
		tab1 = new MyModSettings_tab1 in this; tab1.Init(this);
		tab2subtab1 = new MyModSettings_tab2subtab1 in this; tab2subtab1.Init(this);
		tab2subtab2 = new MyModSettings_tab2subtab2 in this; tab2subtab2.Init(this);

		super.Init();
	}

	public function ReadSettings() : void
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		tab1.option = StringToInt(ReadSettingValue(config, 'MODtab1', 'MODoption'), 0);
		tab1.sliderFloat = StringToFloat(ReadSettingValue(config, 'MODtab1', 'MODsliderFloat'), 0.0);
		tab1.sliderInt = StringToInt(ReadSettingValue(config, 'MODtab1', 'MODsliderInt'), 0);
		tab1.toggle = StringToBool(ReadSettingValue(config, 'MODtab1', 'MODtoggle'));
		tab1.version = StringToFloat(ReadSettingValue(config, 'MODtab1', 'MODversion'), 0.0);

		tab2subtab1.anotherSlider = StringToFloat(ReadSettingValue(config, 'MODtab2subtab1', 'anotherSlider'), 0.0);

		tab2subtab2.anotherToggle = StringToBool(ReadSettingValue(config, 'MODtab2subtab2', 'anotherToggle'));


		super.ReadSettings();
	}

	public function WriteSettings() : void
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		WriteSettingValue(config, 'MODtab1', 'MODoption', IntToString(tab1.option));
		WriteSettingValue(config, 'MODtab1', 'MODsliderFloat', FloatToString(tab1.sliderFloat));
		WriteSettingValue(config, 'MODtab1', 'MODsliderInt', IntToString(tab1.sliderInt));
		WriteSettingValue(config, 'MODtab1', 'MODtoggle', BoolToString(tab1.toggle));
		WriteSettingValue(config, 'MODtab1', 'MODversion', FloatToString(tab1.version));

		WriteSettingValue(config, 'MODtab2subtab1', 'anotherSlider', FloatToString(tab2subtab1.anotherSlider));

		WriteSettingValue(config, 'MODtab2subtab2', 'anotherToggle', BoolToString(tab2subtab2.anotherToggle));


		super.WriteSettings();
	}

	public function ResetSettingsToDefault() : void
	{
		tab1.ResetToDefault();
		tab2subtab1.ResetToDefault();
		tab2subtab2.ResetToDefault();
	}

	public function ShouldResetSettingsToDefaultOnInit() : bool
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		return config.GetVarValue('MODtab1','MODoption') == "";
	}
}

class MyModSettings_tab1 extends ISettingsGroup
{
	public var option : int;
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

