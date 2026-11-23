# pokeget

Display sprites of Pokemon in your terminal.

## Usage
- `pokeget pokemon_name` : print a sprite of a given pokemon
- `pokeget mr-mime` : print Mr. Mime (note the use of `-` instead of spaces)
- `pokeget gengar -m` : print Mega Gengar
- `pokeget random -s` : print a random shiny Pokemon
- `pokeget meowth -a --hide-name` : print Alolan Meowth, without printing the Pokemon's name
- `((RANDOM%4096 == 0)) && pokeget random --shiny || pokeget random` : print a random Pokemon with 1/4096 chance to be shiny

## Categories
documents, desktop, shell

## Links
- More information: https://github.com/talwat/pokeget-rs
- tldr page: https://tldr.inbrowser.app/pages/common/pokeget
