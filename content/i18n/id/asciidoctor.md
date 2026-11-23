# asciidoctor

Ubah isi berkas AsciiDoc ke dalam format berkas layak publikasi.

## Usage
- `asciidoctor jalan/menuju/berkas.adoc` : ubah suatu berkas `.adoc` ke dalam format HTML (format berkas luaran secara default)
- `asciidoctor -a stylesheet=jalan/menuju/stylesheet.css jalan/menuju/berkas.adoc` : ubah suatu berkas `.adoc` ke dalam format HTML dengan menggunakan suatu berkas definisi penggayaan (stylesheet) CSS
- `asciidoctor -e jalan/menuju/berkas.adoc` : ubah suatu berkas `.adoc` menuju format HTML layak semat (embeddable), hanya bangunkan isi tag body HTML
- `asciidoctor -b pdf -r asciidoctor-pdf jalan/menuju/berkas.adoc` : ubah suatu berkas `.adoc` menuju PDF melalui pustaka pendukung `asciidoctor-pdf`

## Links
- More information: https://docs.asciidoctor.org/asciidoctor/latest/cli/man1/asciidoctor/
