```
TW3 Settings Framework Parser 0.2.0
Parses a mod menu XML file and outputs witcher script code representing settings of this menu

USAGE:
    settings_parser [OPTIONS] --file <XML_FILE_PATH> --master <SETTINGS_MASTER_NAME>

OPTIONS:
    -f, --file <XML_FILE_PATH>
            Path to the menu xml file. (REQUIRED)

    -m, --master <SETTINGS_MASTER_NAME>
            Name to use for the settings master class. (REQUIRED)

    -v, --mod-version <MOD_VERSION>
            Version of the mod in format. Format is arbitrary, but preferably a real number
            [default: 1.0]

    -o, --output <OUTPUT_WS_FILE_PATH>
            Path of the WitcherScipt output file, by default it's made from the menu xml file name
            in the same directory

        --omit-prefix <OMIT_PREFIX>
            Prefix to omit from groups and vars when generating code. Case sensitive

        --default-preset-keyword <DEFAULT_PRESET_KEYWORD>
            Keyword used in default presets' display_name. Used to deduce IDs of default presets for
            config groups so they can be used in ResetToDefault() methods. If won't find default
            preset or any preset at that will use 0 as the preset ID. Case insensitive [default:
            default]

    -h, --help
            Print help information

    -V, --version
            Print version information
```