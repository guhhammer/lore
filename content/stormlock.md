# stormlock

Centralized locking system.

## Usage
- `stormlock acquire resource` : acquire a lease for resource
- `stormlock release resource lease_id` : release the given lease for the given resource
- `stormlock current resource` : show information on the current lease for a resource, if any
- `stormlock is-held resource lease_id` : test if a lease for given resource is currently active

## Links
- More information: https://github.com/tmccombs/stormlock#usage
- tldr page: https://tldr.inbrowser.app/pages/common/stormlock
