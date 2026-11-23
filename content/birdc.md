# birdc

BIRD remote control. Retrieve information like routes from bird and perform configurations during runtime.

## Usage
- `birdc` : open the remote control console
- `birdc configure` : reload the configuration without restarting BIRD
- `birdc show status` : show the current status of BIRD
- `birdc show protocols` : show all configured protocols
- `birdc show protocols upstream1 all` : show all details about a protocol
- `birdc "show route where bgp_path ~ 4242120045"` : show all routes that contain a specific AS number
- `birdc show route primary` : show all best routes
- `birdc show route for fd00:/8 all` : show all details of all routes from a given prefix

## Categories
network

## Links
- More information: https://bird.network.cz/?get_doc&v=30&f=bird-4.html
- tldr page: https://tldr.inbrowser.app/pages/common/birdc
