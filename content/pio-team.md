# pio team

Manage PlatformIO teams.

## Usage
- `pio team create --description description organization_name:team_name` : create a new team with the specified description
- `pio team destroy organization_name:team_name` : delete a team
- `pio team add organization_name:team_name username` : add a new user to a team
- `pio team remove organization_name:team_name username` : remove a user from a team
- `pio team list` : list all teams that the user is part of and their members
- `pio team list organization_name` : list all teams in an organization
- `pio team update --name new_team_name organization_name:team_name` : rename a team
- `pio team update --description new_description organization_name:team_name` : change the description of a team

## Categories
users

## Links
- More information: https://docs.platformio.org/en/latest/core/userguide/team/
- tldr page: https://tldr.inbrowser.app/pages/common/pio-team
