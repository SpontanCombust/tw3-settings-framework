# XML specification
This document lists all the custom attribute data that the parser expects to find inside configuration menu XML file.

- **UserConfig**:
  - `msfClass` * - name of the settings master class,
  - `msfVersion` - version of the mod; format is arbitrary, but preferably a real number [default: 1.0],
  - `msfPrefix` - determines the mod prefix to be omitted from id and displayName attributes when parsing them into names used by generated class (e.g. if Var's id is "MODslider" and the prefix is "MOD", variable in WitcherScript will be called "slider"); multiple values can be given by using semicolon (;) as delimiter,
- **Group**:
  - `msfVariable` - custom name of the instance of settings group in WS [default: {id}]; overrides `msfPrefix` behaviour,
- **Var**
  - `msfVariable` - custom name of the instance of settings variable in WS [default: {id}]; overrides `msfPrefix` behaviour,
- **PresetsArray**
  - `msfDefault` - index of the default preset [default: 0]

\* - required