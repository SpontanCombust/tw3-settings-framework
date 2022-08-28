<img src="doc/banner.jpg" alt="Logo"/>

<br>

[Synopsis](#synopsis)

[Overview](#overview)

[Instructions for mod users](#instructions-for-mod-users)

[Instructions for mod developers](#instructions-for-mod-developers)

[Remarks](#remarks)

[Documentation](#documentation)

---

## Synopsis
Framework for modding Witcher 3 that simplifies the pipeline of using mod settings by generating code based directly on mod menu XML and providing extendible interface for reacting to changes of settings.

## Overview
Witcher 3 allows for creating custom mod menus which provide an interface between the mod developer and the end user to further customize their experience. The way settings are accessed in game's scripts is not complicated, but can often result in very boilerplaty code and situations when settings can be fetched improperly simply due to some typo. This framework is trying to midigate that.

First part of this framwork is the menu xml parser. It turns the mod menu xml into WitcherScript code which mirrors the structure of customisable variables in the xml and assures on compile time that the developer uses their variables properly - no way for typos or bad type cast.

Second part of this framework is WitcherScript code that updates generated by parser settings class and even notifies you about it.

## Instructions for mod users

1. Read [remarks](#remarks)
2. Go to the [releases page](https://github.com/SpontanCombust/tw3-settings-framework/releases)
3. Download `TW3_Mod_Settings_Framework_modSettingsFramework.zip` from the newest version or version specified by the developer
4. Install it like any other Witcher 3 mod by dropping what's inside into your `Mods` folder
5. Use Script Merger to solve any conflicts

## Instructions for mod developers

1. Read [remarks](#remarks)


2. Install `TW3_Mod_Settings_Framework_modSettingsFramework.zip` dependency from the [releases page](https://github.com/SpontanCombust/tw3-settings-framework/releases) and use script merger if necessary


3. Download `TW3_Mod_Settings_Framework_Parser.zip` and unpack it anywhere. Use the `settings_parser.exe` program with the menu xml of your mod

```shell
./settings_parser -f ../../doc/mod-menu.xml -m MyModSettings -v 1.23 --omit-prefix=MOD
Group MODtab2 at line 40, column 6 has no vars and will be ignored.
Successfully parsed ../../doc/mod-menu.xml into ../../doc/mod-menu.ws
```
Parser at minimum takes a path to the menu xml file (`-f` flag) and a name you wish your settings class to have (`-m` flag).
To see all the possible options that can be used with the parser use the `--help` (shorter `-h`) option or check the latest [parser CLI specification](doc/cli_specification.md).

[Example mod xml](doc/mod-menu.xml)

[Generated WitcherScript code](doc/mod-menu.ws)

<br>

4. Add generated script file to your mod structure 
   

5. Let the framework know about your settings class

```ts
public var settings : MyModSettings;

mySettings = new MyModSettings in thePlayer;

GetSettingsMasterRegistry().AddSettings(mySettings, 'MyModSettings');
```
First argument is the settings object itself.
Second argument is the ID you want this object to be identified with.

The `AddSettings()` function needs to be used only once. It is up to you when and where you use it. It can be done with Bootstrap or for example in player's OnSpawned function, your choice. The registry will hold the settings object through the entire time the game runs and even if you try to add these settings multiple times with the same ID given, the framework will detect it and won't add a duplicate.


If you don't want to you don't need to store the settings object yourself. You can use registry's GetSettings() method to get access to your settings object.
```ts
// creating the object without assigning it anywhere
GetSettingsMasterRegistry().AddSettings(new MyModSettings in theGame, 'MyModSettings');

...

var mySettings : MyModSettings;

mySettings = (MyModSettings)GetSettingsMasterRegistry().GetSettings('MyModSettings');

if(mySettings)
{
	if(mySettings.tab1.option == 1)
	{
		...
	}
}
```

<br>

6. Use the settings object in your mod

```js
if(thePlayer.mySettings.tab1.toggle)
{
	doSomething();
}
```
From the moment the settings object is added to the registry it gets updated whenever user changes settings in the menu. You do not need to refresh said object yourself, but if you really need to you can do so by calling `ReadSettings()` on it.

It is also possible to change values of these variables and use it to write the data back into game configuration.
```js
mySettings.tab2subtab1.anotherSlider = 0.8;
mySettings.tab2subtab2.anotherToggle = false;

mySettings.WriteSettings();
```

<br>

7. Extend settings master

If the basic functionality that framework classes provide is not enough for you you can extend the settings class generated by the parser and use that child class instead.
The most common usage of this would be overriding `ReadSettings()` method to run arbitrary code whenever the settings class gets updated. To see all the functions available for overriding refer to the [class specification](doc/class_specification.md) or code itself.


## Remarks
WitcherScript side of the framework was made using scripts from [Community Patch - Base](https://www.nexusmods.com/witcher3/mods/3652), so you should need to install it as well. Due to this it _may not_ be possible to be used with Enhanced Edition as that does not use CPB. Seperate compatibility versions may appear in the future, but for now if merging modSettingsFramework with your mod setup produces nasty results you may need to hold off with using it for now.


## Documentation
[WitcherScript class specification](doc/class_specification.md)

[Parser CLI specification](doc/cli_specification.md)