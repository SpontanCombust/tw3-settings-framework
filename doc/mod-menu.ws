// Code generated using Mod Settings Framework & Utilites v0.1.0 by SpontanCombust

class MyModSettings extends ISettingsMaster
{
	public var tab1 : MyModSettings_tab1;
	public var tab2subtab1 : MyModSettings_tab2subtab1;
	public var tab2subtab2 : MyModSettings_tab2subtab2;

	public function ReadSettings()
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		tab1.option = StringToInt(config.GetVarValue('MODtab1', 'MODoption'), 0);
		tab1.sliderFloat = StringToFloat(config.GetVarValue('MODtab1', 'MODsliderFloat'), 0.0);
		tab1.sliderInt = StringToInt(config.GetVarValue('MODtab1', 'MODsliderInt'), 0);
		tab1.toggle = config.GetVarValue('MODtab1', 'MODtoggle');

		tab2subtab1.anotherSlider = StringToFloat(config.GetVarValue('MODtab2subtab1', 'anotherSlider'), 0.0);

		tab2subtab2.anotherToggle = config.GetVarValue('MODtab2subtab2', 'anotherToggle');

	}

	public function WriteSettings()
	{
		var config : CInGameConfigWrapper;
		config = theGame.GetInGameConfigWrapper();

		config.SetVarValue('MODtab1', 'MODoption', IntToString(tab1.option));
		config.SetVarValue('MODtab1', 'MODsliderFloat', FloatToString(tab1.sliderFloat));
		config.SetVarValue('MODtab1', 'MODsliderInt', IntToString(tab1.sliderInt));
		config.SetVarValue('MODtab1', 'MODtoggle', tab1.toggle);

		config.SetVarValue('MODtab2subtab1', 'anotherSlider', FloatToString(tab2subtab1.anotherSlider));

		config.SetVarValue('MODtab2subtab2', 'anotherToggle', tab2subtab2.anotherToggle);

		theGame.SaveUserSettings();
	}
}

struct MyModSettings_tab1
{
	var option : int;
	var sliderFloat : float;
	var sliderInt : int;
	var toggle : bool;
}

struct MyModSettings_tab2subtab1
{
	var anotherSlider : float;
}

struct MyModSettings_tab2subtab2
{
	var anotherToggle : bool;
}
