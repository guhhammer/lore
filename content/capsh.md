# capsh

Capability shell wrapper.

## Usage
- `capsh [OPTION]...` : the general form, as the manual gives it
- `capsh --print` : display prevailing capability and related state
- `capsh --current` : display prevailing capability state, 1e capabilities and IAB vector
- `capsh --caps=cap-set` : set the prevailing process capabilities to those specified by cap-set
- `capsh --drop=cap-list` : remove the listed capabilities from the prevailing bounding set

## Categories
shell
