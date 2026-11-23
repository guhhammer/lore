# latexdiff

Determine differences between two LaTeX files.

## Usage
- `latexdiff old.tex new.tex >diff.tex` : determine changes between different versions of a LaTeX file (the resulting LaTeX file can be compiled to show differences underlined)
- `latexdiff --type=BOLD old.tex new.tex >diff.tex` : determine changes between different versions of a LaTeX file by highlighting differences in boldface
- `latexdiff --math-markup=fine --graphics-markup=both old.tex new.tex >diff.tex` : determine changes between different versions of a LaTeX file, and display minor changes in equations with both added and deleted graphics

## Categories
documents, files, version-control

## Links
- More information: https://ctan.org/pkg/latexdiff
- tldr page: https://tldr.inbrowser.app/pages/common/latexdiff
