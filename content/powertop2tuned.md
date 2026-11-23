# powertop2tuned

Create TuneD profiles from PowerTOP recommendations. Part of `tuned-utils`.

## Usage
- `sudo powertop2tuned profile_name` : create a new TuneD profile based on current PowerTOP recommendations
- `sudo powertop2tuned -e profile_name` : create a new profile and enable most recommendations automatically
- `sudo powertop2tuned -i path/to/report.html profile_name` : create a profile from an existing PowerTOP HTML report
- `sudo powertop2tuned -o path/to/directory profile_name` : create a new profile in a specific directory
- `sudo powertop2tuned -n profile_name` : create a new profile without merging the active profile
- `sudo powertop2tuned -m existing_profile profile_name` : merge recommendations with a specific existing profile
- `sudo powertop2tuned -f profile_name` : force overwrite an existing profile directory

## See also
- powertop
- tuned-adm

## Categories
users, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/powertop2tuned
