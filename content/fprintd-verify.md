# fprintd-verify

Verify fingerprints against the database.

## Usage
- `fprintd-verify` : verify all stored fingerprints for the current user
- `fprintd-verify -f left-thumb|left-index-finger|left-middle-finger|left-ring-finger|left-little-finger|right-thumb|...` : verify a specific fingerprint for the current user
- `fprintd-verify username` : verify fingerprints for a specific user
- `fprintd-verify -f finger_name username` : verify a specific fingerprint for a specific user
- `fprintd-verify --g-fatal-warnings` : fail the process if a fingerprint doesn't match with ones stored in the database for the current user
- `fprintd-verify -h` : display help

## Categories
users, databases

## Links
- More information: https://manned.org/fprintd-verify
- tldr page: https://tldr.inbrowser.app/pages/linux/fprintd-verify
