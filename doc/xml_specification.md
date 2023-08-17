# XML specification
This document lists all the custom attribute data that the parser expects to find inside configuration menu XML file.

- **UserConfig**:
  - `msfClass` * - name of the settings master class,
  - `msfVersion` - version of the mod; format is arbitrary, but preferably a real number [default: 1.0],
- **Group**:
  - `msfVariable` - custom name of the instance of settings group in WS [default: {id}],
- **Var**
  - `msfVariable` - custom name of the instance of settings variable in WS [default: {id}],
- **PresetsArray**
  - `msfDefault` - index of the default preset [default: 0]

\* - required