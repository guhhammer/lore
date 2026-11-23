# acyclic

Jadikan gambaran grafik berarah (dalam Graphviz) menjadi asiklik dengan memutarbalikkan beberapa garis tepi.

## Usage
- `acyclic path/to/input.gv >path/to/output.gv` : ubah grafik berarah menjadi asiklik dengan membalikkan beberapa garis tepi
- `acyclic -v -n path/to/input.gv` : cari tahu apakah grafik tersebut bersifat asiklik, memiliki siklus, atau tidak berarah, sehingga tidak menghasilkan output
- `acyclic -?` : tampilkan bantuan

## Links
- More information: https://graphviz.org/pdf/acyclic.1.pdf
