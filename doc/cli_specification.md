```
TW3 Settings Framework Parser 0.6.0
Parses a mod menu XML file and outputs witcher script code representing settings of this menu

USAGE:
    settings_parser [OPTIONS] <XML_FILE_PATH>

ARGS:
    <XML_FILE_PATH>    Path to the menu xml file. (REQUIRED)

OPTIONS:
    -o, --output <OUTPUT_WS_FILE_PATH>
            Path of the WitcherScipt output file, by default it's made from the menu xml file name
            in the same directory

        --strict-enums
            Causes the parser to throw an error if it will find occurances options that would
            generate unified enum type. More about unified enums in `doc/details.md`

        --no-getter
            Prevents the settings object getter convenience function from being generated

    -h, --help
            Print help information

    -V, --version
            Print version information
```
