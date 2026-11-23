# gh search

Search across all of GitHub.

## Usage
- `gh search code keyword1 keyword2 ...` : search for code containing specific keywords
- `gh search issues "search_phrase"` : search for issues with a specific phrase
- `gh search commits --author username` : search commits by a specific author
- `gh search prs --assignee @me --state open` : search pull requests assigned to yourself that are still open
- `gh search repos --owner org_name --topic topic_name` : search repositories in an organization by topic
- `gh search issues -- "search_query -label:label_name"` : search issues without a certain label (Unix-like systems)
- `gh --% search issues -- "search_query -label:label_name"` : search issues without a certain label (PowerShell)
- `gh search subcommand -w query` : open the search query in the web browser

## Links
- More information: https://cli.github.com/manual/gh_search
- tldr page: https://tldr.inbrowser.app/pages/common/gh-search
