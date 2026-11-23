# qtgrace

Display, plot, analyze 2D data. A Qt reimplementation of Grace (<https://plasma-gate.weizmann.ac.il/Grace/>), sharing its command-line interface.

## Usage
- `qtgrace path/to/file1.dat path/to/file2.dat ...` : plot one or more data files, plotting only the first two columns in each file as X Y
- `qtgrace -nxy path/to/file1.dat -nxy path/to/file2.dat ...` : plot one or more data files, plotting all columns in each file as X Y1 Y2
- `qtgrace -log x path/to/file1.dat path/to/file2.dat ...` : plot one or more data files with a logarithmically scaled x-axis
- `qtgrace -log xy -settype xydy path/to/file.dat` : plot a data file with a logarithmic scale on both axes, with data formatted as X Y DY
- `qtgrace -help` : display help

## Categories
files, network, desktop

## Links
- More information: https://sourceforge.net/projects/qtgrace/
- tldr page: https://tldr.inbrowser.app/pages/linux/qtgrace
