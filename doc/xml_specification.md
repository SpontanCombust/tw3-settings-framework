# XML specification
This document lists all the custom data that the parser expects to find inside configuration menu XML file.

- **UserConfig**:
  - `msfClass` * - name of the settings master class,
  - `msfVersion` - version of the mod; format is arbitrary, but preferably a real number [default: 1.0],
- **Group**:
  - `msfClass` - name of the group class [default: {SettingsMasterName}_{GroupId}],  

\* - required