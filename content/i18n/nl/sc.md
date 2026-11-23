# sc

Communiceer met de Service Control Manager en services.

## Usage
- `sc query service_naam` : toon de status van een service (geen servicenaam zal alle services tonen)
- `sc start service_naam` : start een service asynchroon
- `sc stop service_naam` : stop een service asynchroon
- `sc create service_naam binpath= pad\naar\service_binary_bestand` : maak een service aan
- `sc delete service_naam` : verwijder een service
- `sc config service_naam type= service_type` : stel het type van een service in
