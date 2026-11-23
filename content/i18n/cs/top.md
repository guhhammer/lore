# top

Zobrazuje dynamické informace o běžících procesech v reálném čase.

## Usage
- `top` : zapnout `top`
- `top -i` : nezobrazovat nečinné nebo zombie procesy
- `top -u jmeno_uzivatele` : zobrazovat pouze procesy vlastněné daným uživatelem
- `top -o nazev_pole` : řadit procesy podle pole
- `top -Hp id_procesu` : zobrazovat individuální vlákna daného procesu
- `top -p $(pgrep -d ',' nazev_procesu)` : zobrazovat pouze procesy s daným(i) PID, předaný jako list rozdělený čárkou. (Normálně byste nevěděli PID z hlavy. Tento příklad vybíra PIDs podle názvu procesu)
- `<?>` : zobrazit nápovědu o interaktivních příkazech

## Links
- More information: https://manned.org/top
