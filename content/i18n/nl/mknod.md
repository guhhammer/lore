# mknod

Maak speciale bestanden voor blok- of tekenapparaten aan.

## Usage
- `sudo mknod pad/naar/apparaat_bestand b groot_apparaatnummer klein_apparaatnummer` : maak een blokapparaat aan
- `sudo mknod pad/naar/apparaat_bestand c groot_apparaatnummer klein_apparaatnummer` : maak een tekenapparaat aan
- `sudo mknod pad/naar/apparaat_bestand p` : maak een FIFO (queue) apparaat aan
- `sudo mknod -Z pad/naar/apparaat_bestand type groot_apparaatnummer klein_apparaatnummer` : maak een apparaatbestand aan met de standaard SELinux-beveiligingscontext
