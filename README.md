# CrabType

A TUI based [alternative](https://thealternative.ch) to [MonkeyType](https://monkeytype.com) written in rust.

## Installation

### Cargo

Simply type

```sh
cargo install crabtype
```

### Nix

To build CrabType for nix just add this derivation to your nix config

```nix
    rustapple = pkgs.callPackage (pkgs.fetchFromGitHub {
        owner = "einsJannis";
        repo = "CrabType";
        rev = "<Git Revision>";
        sha256 = "<Git SHA>";
    }) {};
```

You can figure out the revision and the sha265 by running the application `nix-prefetch-git`

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
