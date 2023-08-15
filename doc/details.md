# Details

## Data types in WitcherScript
Var's displayType and some other factors determine what type does the corresponding variable in WitcherScript take:
- **TOGGLE** -> `bool`,
- **SLIDER** -> `int` or `float` depending on whether said slider can yield fractional numbers, for example "SLIDER;0;100;100" will be assigned int type, because value step is equal to 1 and slider can yield only integer values, but "SLIDER;0;100;200" will be assigned float type, because it can yield non-integer values like 0.5,
- **OPTIONS** -> `int` or custom `enum` type depending on `--option-parsing-mode` option in CLI. More about it [here](#a-word-or-two-about-options),
- **SUBTLE_SEPARATOR** -> vars with this type are ignored in the process of parsing to WitcherScript.


## A word or two about parsing options

### Basics
Since version v0.5 of the framework the parser can generate enum types based on "OPTIONS" type vars in config xml.
General rules on how the parser does this:
1. `displayName` attributes in each Option node are checked and a common prefix is obtained if possible
2. Name of the enum type is constructed as such: `{SettingsMasterName}_{CommonDisplayNamePrefix}` if the common prefix exists. If it does not the `id` attribute of var is used: `{SettingsMasterName}_{VarId}`
3. Enum values' names are obtained as such: `{EnumTypeName}{Suffix}` where `Suffix` is the remaining part of `displayName` without the common prefix.

When `id`s and `displayName`s are parsed values from the `--omit-prefix` CLI option are also applied.

Considering the fact that multiple options can have the same or similar set of possible values a few features have been put in place:
1. For options with the same deduced enum type only one enum definition is generated
2. When `--option-parsing-mode` CLI option takes `enums-strict` value all xml options that have the same common prefix need to have the exact same set of possible values in the same order.
3. When `--option-parsing-mode` CLI option takes `enums` value (which is the default) and multiple options have the same deduced type, but different sets of values, a [unified](#unified-enums) enum is constructed.

### Unified enums
Unified enums are what this framework calls enum types that are assigned to options that share the same common prefix in their possible values. Example: let's say we have bunch of options that dictate the "quality" of graphics. Possible values in total are "Low", "Medium", "High" and "Ultra". Not all settings might need to use all of them. Some of them may only go up to "High" and others might start from "Medium". All of them however would be attributed the same enum type in the WitcherScript. So this feature allows all of these settings to be handled using the same enum type. 
Example of unified enums can be found in [Monster of the Week](../samples/MonsterOfTheWeek) example.

If a variable of a unified enum type in WitcherScript gets assigned a value it is not supposed to take (e.g. a setting that cannot have "Low" quality gets assigned such) before this value will be saved into user config it will be corrected in the `ValidateSettings` function. The current behaviour is to assign it the first valid value (in this case "Medium" quality).

If a unified enum type exists for the settings class, an extra set of functions are generated, which names start with `EnumValueMapping`. They essentially make all of this just work and allow the conversion from user config integer value to enum value in WitcherScript and vice versa.