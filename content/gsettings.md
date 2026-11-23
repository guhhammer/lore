# gsettings

Query and modify dconf settings with schema validation.

## Usage
- `gsettings set org.example.schema example_key value` : set the value of a key. Fails if the key doesn't exist or the value is out of range
- `gsettings get org.example.schema example_key` : print the value of a key or the schema-provided default if the key has not been set in `dconf`
- `gsettings reset org.example.schema example_key` : unset a key, so that its schema default value will be used
- `gsettings list-recursively` : display all (non-relocatable) schemas, keys, and values
- `gsettings list-recursively org.example.schema` : display all keys and values (default if not set) from one schema
- `gsettings range org.example.schema example_key` : display schema-allowed values for a key (helpful with enum keys)
- `gsettings describe org.example.schema example_key` : display the human-readable description of a key

## Categories
desktop, databases

## Links
- More information: https://manned.org/gsettings
- tldr page: https://tldr.inbrowser.app/pages/linux/gsettings
