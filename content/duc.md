# duc

A collection of tools for indexing, inspecting, and visualizing disk usage. Duc maintains a database of accumulated sizes of directories of the filesystem, allowing queries in this database, or creating fancy graphs to show where data is.

## Usage
- `duc index /usr` : index the `/usr` directory, writing to the default database location `~/.duc.db`
- `duc ls -Fg /usr/local` : list all files and directories under `/usr/local`, showing relative file sizes in a graph
- `duc ls -Fg -R /usr/local` : list all files and directories under `/usr/local` using treeview recursively
- `duc gui /usr` : start the graphical interface to explore the filesystem using sunburst graphs
- `duc ui /usr` : run the ncurses console interface to explore the filesystem
- `duc info` : dump database info

## Categories
databases, disks, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/duc
