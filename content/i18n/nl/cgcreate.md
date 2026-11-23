# cgcreate

Maak cgroups, gebruikt om bronnen te beperken, te meten en te regelen die door processen worden gebruikt. `cgroups` types kunnen een van `memory`, `cpu`, `net_cls`, etc. zijn.

## Usage
- `cgcreate -g groep_type:groepsnaam` : maak een nieuwe groep
- `cgcreate -g groep_type1,groep_type2:groepsnaam` : maak een nieuwe groep met meerdere cgroep typen
- `mkdir /sys/fs/cgroup/groep_type/groepsnaam/subgroep_naam` : maak een subgroep

## Links
- More information: https://manned.org/cgcreate
