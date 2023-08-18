# XML specification
This document lists all the custom attribute data that the parser expects to find inside configuration menu XML file.

- **UserConfig**:
  - `msfClass` * - name of the settings master class,
  - `msfVersion` - version of the mod; format is arbitrary, but preferably a real number [default: 1.0],
- **Group**:
  - `msfVariable` - custom name of the instance of settings group in WS, by default this name is the 'id' attribute,
- **Var**
  - `msfVariable` - custom name of the instance of settings variable in WS, by default this name is the 'id' attribute,
- **OptionsArray**
  - `msfIsEnum` - determines whether this OptionsArray should be parsed as enum or int type [default: "true"]
  - `msfEnum` ** - custom name of the enum type, by default it is made as {SettingsMasterClass}_{CommonDisplayNamePrefix}, where 'CommonDisplayNamePrefix' is the common beginning part of displayName attributes in subject Option nodes; if there is no common prefix between displayNames in Options this attribute is required; ignored if `msfIsEnum` is "false"; more about parsing enums in [details](./details.md),
- **Option**
  - `msfEnumValue` ** - custom suffix for the enum value if `msfEnum` is specified; by default these values are made as {SettingsMasterClass}_{displayName},
- **PresetsArray**
  - `msfDefault` - index of the default preset [default: 0].

\* - required <br/>
\*\* - required conditionally