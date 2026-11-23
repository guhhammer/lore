# wpctl

Beheer WirePlumber, een sessie- en beleidsbeheerder voor PipeWire. Opmerking: je kunt de speciale naam `@DEFAULT_SINK@` gebruiken in plaats van `id` om de standaard sink te beheren.

## Usage
- `wpctl status` : toon alle objecten die door WirePlumber worden beheerd
- `wpctl inspect object_id` : toon alle eigenschappen van een object
- `wpctl set-default object_id` : stel een object in als standaard binnen zijn groep
- `wpctl get-volume sink_id` : verkrijg het volume van een sink
- `wpctl set-volume sink_id n%|n%+|n%-` : stel het volume van een sink in op `n` procent of verhoog of verlaag het met `n`
- `wpctl list audio|video devices|sinks|sources` : filter objecten op type
- `wpctl set-volume -l 1 sink_id n%-` : verhoog het volume van een sink met `n` procent, maar begrens het volume onder 100%
- `wpctl set-mute @DEFAULT_SINK|SOURCE@ 1|0|toggle` : demp de standaard audio sink of source of hef de demping op (1 is dempen, 0 is opheffen)

## Links
- More information: https://pipewire.pages.freedesktop.org/wireplumber/
