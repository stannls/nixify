# Nixify
A simple cli to convert existing dotfiles into nix syntax

## Usage
Specify the name of the program with the name flag and the format of the config file with the format flag. Currently supported are json, toml and yaml. The program will generate a nix file that contains all specified config keys in home-manager syntax.
