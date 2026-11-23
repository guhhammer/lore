# ndctl

Utility for managing Non-Volatile DIMMs.

## Usage
- `ndctl create-namespace --mode=fsdax` : create an `fsdax` mode namespace
- `ndctl create-namespace --reconfigure=namespaceX.Y --mode=raw` : change the mode of a namespace to `raw`
- `ndctl check-namespace --repair namespaceX.Y` : check a sector mode namespace for consistency, and repair if needed
- `ndctl list --namespaces --regions --buses --idle` : list all namespaces, regions, and buses (including disabled ones)
- `ndctl list -vvv --namespace=namespaceX.Y` : list a specific namespace and include lots of additional information
- `ndctl monitor --bus=ACPI.NFIT` : run a monitor to watch for SMART health events for NVDIMMs on the `ACPI.NFIT` bus
- `ndctl destroy-namespace --force namespaceX.Y` : remove a namespace (when applicable) or reset it to an initial state

## Links
- More information: https://manned.org/ndctl
- tldr page: https://tldr.inbrowser.app/pages/linux/ndctl
