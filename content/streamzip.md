# streamzip

Create a zip file from stdin.

## Usage
- `producer | streamzip [opts] | consumer producer | streamzip [opts] -zipfile=output.zip` : the general form, as the manual gives it
- `streamzip -zip64` : create a Zip64-compliant zip container
- `streamzip -zipfile=F` : write zip container to the filename "F"
- `streamzip -member-name=M` : this option is used to name the "file" in the zip container
- `streamzip -stream` : ignored when writing to "stdout"

## Categories
archives, files, containers

## Links
- Documentation: https://rt.cpan.org/Public/Dist/Display.html?Name=IO-Compress
