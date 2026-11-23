# git-upload-pack

Send objects packed back to git-fetch-pack.

## Usage
- `git-upload-pack [--[no-]strict] [--timeout=<n>] [--stateless-rpc] [--advertise-refs] <directory>` : the general form, as the manual gives it
- `git-upload-pack --timeout=<n>` : interrupt transfer after <n> seconds of inactivity
- `git-upload-pack --stateless-rpc` : perform only a single read-write cycle with stdin and stdout

## Categories
version-control, network
