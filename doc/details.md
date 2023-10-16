# Details

## Data types in WitcherScript
Var's displayType and some other factors determine what type does the corresponding variable in WitcherScript take:
- **TOGGLE** -> `bool`,
- **SLIDER** -> `int` or `float` depending on whether said slider can yield fractional numbers, for example "SLIDER;0;100;100" will be assigned int type, because value step is equal to 1 and slider can yield only integer values, but "SLIDER;0;100;200" will be assigned float type, because it can yield non-integer values like 0.5,
- **OPTIONS** -> `int` or custom `enum` type depending on `msfIsEnum` value. More about it in [enums](#enums) section,
- **SUBTLE_SEPARATOR** -> vars with this type are ignored in the process of parsing to WitcherScript.


## Custom XML data
The game reads XML config files and takes only the data that it needs without applying strict schema rules onto them. What this means is that alongside the well known data that the game expects we can put extra data that only the framework's parser needs without risking the game to throw any errors.

Most of the data that has previously been passed into the parser through CLI now is done by using custom XML attributes.
The full specification describing it together with which attributes are necessary can be found in [xml specification](./xml_specification.md).


## Default values on first use
When initialising the settings for your mod for the very first time in game the framework applies a default preset for each settings Group. Therefore if you want a specific set of values to be set you should create PresetsArrays for your Groups. By default the preset with index 0 is picked. If you want other one to be the default, use `msfDefault` attribute in PresetsArray node.

## Var validation
After reading from and before writing to user configuration during the game values of the parsed class are corrected to adhere to their XML constraints, for example a variable corresponding to the slider var in XML will have its value clamped between minimal and maximal possible values.
It is possible to disable this validation for the entire class or singular groups and vars. To do it use the `msfValidate` attribute.


## Enums

### Basics
Since version v0.5 of the framework the parser can generate enum types based on "OPTIONS" type vars in config xml.
General rules on how the parser does this:
- The old way:
  1. `displayName` attributes in each Option node are checked and a common prefix is obtained if possible.
  2. Name of the enum type is constructed as such: `{SettingsMasterName}_{CommonDisplayNamePrefix}` if the common prefix exists. Not having such prefix leads to an error.
  3. Enum values' names are obtained as such: `{EnumTypeName}{Suffix}` where `Suffix` is the remaining part of `displayName` without the common prefix.
   
When `id`s and `displayName`s are parsed, their beginnings are stripped with values from `msfPrefix` attribute (if any is specified).

- The new way using custom attributes:
  1. Name enum type in OptionsArray node using `msfEnum="EDifficulty"`
  2. Name enum value suffixes in subsequent Option nodes using `msfEnumValue=Easy`, `msfEnumValue=Hard` etc.


Considering the fact that multiple options can have the same or similar set of possible values a few features have been put in place:
1. For options with the same deduced enum type only one enum definition is generated
2. If `--strict-enums` CLI flag is enabled all xml options that have the same common prefix need to have the exact same set of possible values in the same order.
3. If `--strict-enums` CLI flag is not enabled (which is the default) and multiple options have the same deduced type, but different sets of values, a [unified](#unified-enums) enum is constructed.

### Unified enums
Unified enums are what this framework calls enum types that are assigned to options that share the same common prefix in their possible values. Example: let's say we have bunch of options that dictate the "quality" of graphics. Possible values in total are "Low", "Medium", "High" and "Ultra". Not all settings might need to use all of them. Some of them may only go up to "High" and others might start from "Medium". All of them however would be attributed the same enum type in the WitcherScript. So this feature allows all of these settings to be handled using the same enum type. 
Example of unified enums can be found in [Monster of the Week](../samples/MonsterOfTheWeek) example.

If a variable of a unified enum type in WitcherScript gets assigned a value it is not supposed to take (e.g. a setting that cannot have "Low" quality gets assigned such) before this value will be saved into user config it will be corrected in the `ValidateSettings` function. The current behaviour is to assign it the first valid value (in this case "Medium" quality).

If a unified enum type exists for the settings class, an extra set of functions are generated, which names start with `EnumValueMapping`. They essentially make all of this just work and allow the conversion from user config integer value to enum value in WitcherScript and vice versa.


## Hooks
Before v1.0 I've distributed modified vanilla scripts and local scripts in a single `modSettingsFramework` package. This was not ideal as I had to release new **framwork version** for each new **game versions** no matter what. 
Now however there are `hooks`. There are variants for most major game versions that you install alongside `modSettingsFramework`, which now only has local scripts that don't modify vanilla ones.


## Breaking changes

### v1.0
- WitcherScript
  - seperated vanilla and local scripts. Before updating delete previous files so there is no repeating code

### v0.6
- XML
  - some custom attributes are required to successfully parse the document (see [Custom XML data](#custom-xml-data))
- CLI
  - removed `--file` argument, replaced it with a positional (you only need to input the XML path itself)
  - removed `--master` argument
  - removed `--mod-version` argument
  - removed `--omit-prefix` argument
  - removed `--default-preset-keyword` argument
  - removed `--option-parsing-mode` argument, added `--strict-enums` flag in its place
  - removed ` --no-var-validation` argument
- WitcherScript
  - renamed `Reset` function in `ISettingsGroup` to `ResetSettings`
  - renamed `ResetToDefault` function in `ISettingsGroup` to `ResetSettingsToDefault`
  - moved `StringToBool` and `BoolToString` from `ISettingsMaster` to `ISettingsGroup`
  - removed `SSettingsMasterRegistryEntry` struct type
  - heavily modified how parsed classes look

### v0.2
- WitcherScript
  - removed `ISettingsReadListener` class
  - removed `AddReadListener` function from `CSettingsMasterRegistry`