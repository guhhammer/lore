# puppet agent

Ruft die Client-Konfiguration eines Puppetservers ab und setzt diese auf dem System um.

## Usage
- `puppet agent --test --server puppetserver_fqdn --serverport port --waitforcert poll_zeit` : registriere die Node bei einem Puppetserver und wende den empfangenen Katalog an
- `puppet agent` : führe den Agenten im Hintergrund aus (nutzt die Einstellungen von `/opt/puppetlabs/puppet/puppet.conf`)
- `puppet agent --test` : führe den Agenten einmal im Vordergrund aus und beende
- `puppet agent --test --noop` : führe den Agenten im Dry-Modus aus
- `puppet agent --test --evaltrace` : protokolliere jede ausgewertete Ressource (selbst wenn sich nichts geändert hat)
- `puppet agent --disable "nachricht"` : deaktiviere den Agenten
- `puppet agent --enable` : aktiviere den Agenten

## Links
- More information: https://github.com/puppetlabs/puppet/blob/main/references/man/agent.md
