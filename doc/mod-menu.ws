// Code generated using Mod Settings Framework v0.5.0 by SpontanCombust & Aeltoth

class MyModSettings extends ISettingsMaster
{
	default modVersion = "1.23";

	public var tab1 : MyModSettings_tab1;
	public var tab2 : MyModSettings_tab2;
	public var tab3 : MyModSettings_tab3;

	protected /* override */ function Parser_Init() : void
	{
		tab1 = new MyModSettings_tab1 in this; tab1.Init(this);
		tab2 = new MyModSettings_tab2 in this; tab2.Init(this);
		tab3 = new MyModSettings_tab3 in this; tab3.Init(this);
	}

	protected /* override */ function Parser_ValidateSettings() : void
	{
		tab1.Validate();
		tab2.Validate();
		tab3.Validate();
	}

	protected /* override */ function Parser_ReadSettings(config : CInGameConfigWrapper) : void
	{
		tab1.Read(config);
		tab2.Read(config);
		tab3.Read(config);
	}

	protected /* override */ function Parser_WriteSettings(config : CInGameConfigWrapper) : void
	{
		tab1.Write(false, config);
		tab2.Write(false, config);
		tab3.Write(false, config);
	}

	protected /* override */ function Parser_ResetSettingsToDefault(config : CInGameConfigWrapper) : void
	{
		tab1.ResetToDefault(false, config);
		tab2.ResetToDefault(false, config);
		tab3.ResetToDefault(false, config);
	}

	protected /* override */ function Parser_ShouldResetSettingsToDefaultOnInit(config : CInGameConfigWrapper) : bool
	{
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

	protected /* override */ function Parser_Validate() : void
	{
		option = (MyModSettings_opt)Clamp((int)option, 0, 2);
		sliderFloat = ClampF(sliderFloat, 0, 1);
		sliderInt = Clamp(sliderInt, 0, 100);
		version = ClampF(version, 0, 100);
	}

	protected /* override */ function Parser_Read(config: CInGameConfigWrapper) : void
	{
		option = (MyModSettings_opt)ReadIntSettingValue(config, 'MODoption');
		sliderFloat = ReadFloatSettingValue(config, 'MODslider1');
		sliderInt = ReadIntSettingValue(config, 'MODslider2');
		toggle = ReadBoolSettingValue(config, 'MODtoggle');
		version = ReadFloatSettingValue(config, 'MODversion');
	}

	protected /* override */ function Parser_Write(config: CInGameConfigWrapper) : void
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

	protected /* override */ function Parser_Validate() : void
	{
		anotherSlider = ClampF(anotherSlider, -100, 100);
	}

	protected /* override */ function Parser_Read(config: CInGameConfigWrapper) : void
	{
		anotherSlider = ReadFloatSettingValue(config, 'anotherSlider');
	}

	protected /* override */ function Parser_Write(config: CInGameConfigWrapper) : void
	{
		WriteFloatSettingValue(config, 'anotherSlider', anotherSlider);
	}
}

class MyModSettings_tab3 extends ISettingsGroup
{
	public var anotherToggle : bool;

	default id = 'MODtab2subtab2';
	default defaultPresetIndex = 0;

	protected /* override */ function Parser_Validate() : void
	{
	}

	protected /* override */ function Parser_Read(config: CInGameConfigWrapper) : void
	{
		anotherToggle = ReadBoolSettingValue(config, 'anotherToggle');
	}

	protected /* override */ function Parser_Write(config: CInGameConfigWrapper) : void
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
