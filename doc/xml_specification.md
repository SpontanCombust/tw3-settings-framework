# XML specification
This document lists all the custom attribute data that the parser expects to find inside configuration menu XML file.

- **UserConfig**:
  - `msfClass` (string) * - name of the settings master class,
  - `msfVersion` (string) - version of the mod; format is arbitrary, but preferably a real number [default: 1.0],
  - `msfPrefix` (string) - determines the mod prefix to be omitted from id and displayName attributes when parsing them into names used by generated class (e.g. if Var's 'id' is "MODslider" and the prefix is "MOD", variable in WitcherScript will be called "slider"); case sensitive, multiple values can be given by using semicolon (;) as delimiter; does not apply to any other custom framework attributes, such as `msfVariable`,
- **Group**:
  - `msfVariable` (string) - custom name of the instance of settings group in WS, by default this name is obtained from 'id' attribute,
  - `msfIgnore` (bool) - determines whether the group should be ignored or not [default: "false"]
- **Var**
  - `msfVariable` (string) - custom name of the instance of settings variable in WS, by default this name is obtained from 'id' attribute,
  - `msfIgnore` (bool) - determines whether the var should be ignored or not [default: "false"]
- **OptionsArray**
  - `msfIsEnum` (bool) - determines whether this OptionsArray should be parsed as enum or int type [default: "true"]
  - `msfEnum` (string) ** - custom name of the enum type, by default it is made as {SettingsMasterClass}_{CommonDisplayNamePrefix}, where 'CommonDisplayNamePrefix' is the common beginning part of displayName attributes in subject Option nodes; if there is no common prefix between displayNames in Options this attribute is required; ignored if `msfIsEnum` is "false"; more about parsing enums in [details](./details.md),
- **Option**
  - `msfEnumValue` (string) ** - custom suffix for the enum value if `msfEnum` is specified; by default these values are made as {SettingsMasterClass}_{displayName},
- **PresetsArray**
  - `msfDefault` (int) - index of the default preset [default: 0].

\* - required <br/>
\*\* - required conditionally