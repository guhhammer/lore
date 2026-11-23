# ansible-doc

Display information on modules installed in Ansible libraries. Display a terse listing of plugins and their short descriptions.

## Usage
- `ansible-doc -l` : list available action plugins (modules)
- `ansible-doc -t become|cache|callback|cliconf|connection|... -l` : list available plugins of a specific type
- `ansible-doc plugin_name` : show information about a specific action plugin (module)
- `ansible-doc -t become|cache|callback|cliconf|connection|... plugin_name` : show information about a plugin with a specific type
- `ansible-doc -s plugin_name` : show the playbook snippet for action plugin (modules)
- `ansible-doc -j plugin_name` : show information about an action plugin (module) as JSON

## Categories
desktop

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-doc.html
- tldr page: https://tldr.inbrowser.app/pages/common/ansible-doc
