// Code generated using Mod Settings Framework v1.0.0 by SpontanCombust & Aeltoth

class MyModSettings extends ISettingsMaster
{
	default modVersion = "1.23";

	public var tab1 : MyModSettings_tab1;
	public var tab2 : MyModSettings_tab2;
	public var tab3 : MyModSettings_tab3;

	protected /* override */ function Parser_Init() : void
	{
		tab1 = new MyModSettings_tab1 in this;
		tab1.Init(this);
		m_groups.PushBack(tab1);

		tab2 = new MyModSettings_tab2 in this;
		tab2.Init(this);
		m_groups.PushBack(tab2);

		tab3 = new MyModSettings_tab3 in this;
		tab3.Init(this);
		m_groups.PushBack(tab3);
	}

	protected /* override */ function Parser_ShouldResetSettingsToDefaultOnInit(config : CInGameConfigWrapper) : bool
	{
		return ReadSettingValue(config, 'MODtab1','MODoption') == "-1";
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

	protected /* override */ function Parser_ValidateSettings() : void
	{
		option = (MyModSettings_opt)Clamp((int)option, 0, 2);
		sliderFloat = ClampF(sliderFloat, 0, 1);
		sliderInt = Clamp(sliderInt, 0, 100);
		version = ClampF(version, 0, 100);
	}

	protected /* override */ function Parser_ReadSettings(config: CInGameConfigWrapper) : void
	{
		option = (MyModSettings_opt)ReadIntSettingValue(config, 'MODoption');
		sliderFloat = ReadFloatSettingValue(config, 'MODslider1');
		sliderInt = ReadIntSettingValue(config, 'MODslider2');
		toggle = ReadBoolSettingValue(config, 'MODtoggle');
		version = ReadFloatSettingValue(config, 'MODversion');
	}

	protected /* override */ function Parser_WriteSettings(config: CInGameConfigWrapper) : void
	{
		WriteIntSettingValue(config, 'MODoption', (int)option);
		WriteFloatSettingValue(config, 'MODslider1', sliderFloat);
		WriteIntSettingValue(config, 'MODslider2', sliderInt);
		WriteBoolSettingValue(config, 'MODtoggle', toggle);
		WriteFloatSettingValue(config, 'MODversion', version);
	}
}

class MyModSettings_tab2 extends ISettingsGroup
{
	public var anotherSlider : float;

	default id = 'MODtab2subtab1';
	default defaultPresetIndex = 0;

	protected /* override */ function Parser_ValidateSettings() : void
	{
		anotherSlider = ClampF(anotherSlider, -1.5, 1.5);
	}

	protected /* override */ function Parser_ReadSettings(config: CInGameConfigWrapper) : void
	{
		anotherSlider = ReadFloatSettingValue(config, 'anotherSlider');
	}

	protected /* override */ function Parser_WriteSettings(config: CInGameConfigWrapper) : void
	{
		WriteFloatSettingValue(config, 'anotherSlider', anotherSlider);
	}
}

class MyModSettings_tab3 extends ISettingsGroup
{
	public var anotherToggle : bool;

	default id = 'MODtab2subtab2';
	default defaultPresetIndex = 0;

	protected /* override */ function Parser_ValidateSettings() : void
	{
	}

	protected /* override */ function Parser_ReadSettings(config: CInGameConfigWrapper) : void
	{
		anotherToggle = ReadBoolSettingValue(config, 'anotherToggle');
	}

	protected /* override */ function Parser_WriteSettings(config: CInGameConfigWrapper) : void
	{
		WriteBoolSettingValue(config, 'anotherToggle', anotherToggle);
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
