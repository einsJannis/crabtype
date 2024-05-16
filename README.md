# CrabType

A TUI based [alternative](https://thealternative.ch) to [MonkeyType](https://monkeytype.com) written in rust.

## Run

```sh
crabtype [options...]
```
### Options
```
-g|--gamemode <file>
```
Changes the game mode to the one defined in `<file>`. This file must contain each possible text, separated by newlines.
```
-u|--user <file>
```
Sets the user file to `<file>`, which is the file which stores the stats in a TOML format.
