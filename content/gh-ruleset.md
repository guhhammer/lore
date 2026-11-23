# gh ruleset

Manage GitHub repository rulesets.

## Usage
- `gh rs ls` : list all rulesets for the current repository
- `gh rs ls -o organization_name` : list all rulesets for a specific organization
- `gh rs check` : check the rules that apply to the current branch
- `gh rs check branch_name -R owner/repository` : check the rules that apply to a specific branch in another repository
- `gh rs view` : interactively select and view a ruleset for the current repository
- `gh rs view ruleset_id` : view a specific ruleset by its ID
- `gh rs view ruleset_id -o organization_name` : view an organization-level ruleset by its ID
- `gh rs ls -R owner/repository -w` : open the list of rulesets for a specific repository in the browser

## Categories
packages

## Links
- More information: https://cli.github.com/manual/gh_ruleset
- tldr page: https://tldr.inbrowser.app/pages/common/gh-ruleset
