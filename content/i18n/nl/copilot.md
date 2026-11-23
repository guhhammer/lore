# copilot

Communiceer met GitHub Copilot.

## Usage
- `copilot` : start de interactieve modus
- `copilot --allow-tool write` : sta alle bestandswijzigingen toe
- `copilot --continue` : hervat de recentste sessie
- `copilot --resume` : hervat een eerdere sessie met behulp van een sessiekiezer
- `copilot --model "gpt-5"` : gebruik een specifiek model
- `copilot --allow-tool 'shell(git:*)' --deny-tool 'shell(git push)'` : sta alle Git-commando's toe, behalve `git push`
- `copilot -p "Repareer de bug in main.js" --allow-all-tools` : voer een prompt direct uit zonder interactieve modus, terwijl `copilot` alle commando's mag uitvoeren

## Links
- More information: https://docs.github.com/en/copilot/concepts/agents/about-copilot-cli
