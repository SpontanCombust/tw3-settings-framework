```
TW3 Settings Framework Parser 0.5.0
Parses a mod menu XML file and outputs witcher script code representing settings of this menu

USAGE:
    settings_parser [OPTIONS] --master <SETTINGS_MASTER_NAME> <XML_FILE_PATH>

ARGS:
    <XML_FILE_PATH>    Path to the menu xml file. (REQUIRED)

OPTIONS:
    -m, --master <SETTINGS_MASTER_NAME>
            Name to use for the settings master class. (REQUIRED)

    -v, --mod-version <MOD_VERSION>
            Version of the mod in format. Format is arbitrary, but preferably a real number
            [default: 1.0]

    -o, --output <OUTPUT_WS_FILE_PATH>
            Path of the WitcherScipt output file, by default it's made from the menu xml file name
            in the same directory

    -p, --omit-prefix <OMIT_PREFIX>
            Prefix to omit from groups and vars when generating code. Case sensitive. Possible
            multiple cases

        --default-preset-keyword <DEFAULT_PRESET_KEYWORD>
            Keyword used in default presets' display_name. Used to deduce IDs of default presets for
            config groups so they can be used in ResetToDefault() methods. If won't find default
            preset or any preset at that will use 0 as the preset ID. Case insensitive [default:
            default]

        --option-parsing-mode <OPTION_PARSING_MODE>
            Controls how OPTION type vars are parsed into WitcherScript
            - ints:
            Treats options vars as regular ints instead of creating custom enum types for them.
            This essentially means the behaviour from before v0.5.
            - enums:
            Parses options vars into enums. Then tries to find vars that have the same set of
            displayName attributes in option node
            and assigns them one common type.
            Requires that displayNames of all option nodes contain some prefix that determines their
            relation.
            If two option arrays contain the same set of possible values they are considered to have
            the same enum type.
            - enums-strict:
            Parses options vars into enums with an exception that having mutliple option arrays
            designated by the same prefix
            but having different sets of values is disallowed. This prevents possible user mistakes
            from happening.  [default: enums] [possible values: ints, enums, enums-strict]

        --no-var-validation
            Disables the generation of code for value correction. After reading from or before
            writing to user config values will no longer be checked if they adhere to the XML, e.g.
            if slider value is in a specified range

        --no-getter
            Prevents the settings object getter convenience function from being generated

    -h, --help
            Print help information

    -V, --version
            Print version information
```
