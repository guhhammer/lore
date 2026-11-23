# jj simplify-parents

Simplify parent edges for the specified revision(s). For example, "A -> B -> C | A -> C" gets simplified to "A -> B -> C".

## Usage
- `jj simplify-parents -r revsets` : simplify parent edges of given revisions
- `jj simplify-parents -s revsets` : simplify parent edges of given revisions and trees of their descendants

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-simplify-parents
- tldr page: https://tldr.inbrowser.app/pages/common/jj-simplify-parents
