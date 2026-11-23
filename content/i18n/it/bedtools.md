# bedtools

Un coltellino svizzero di strumenti per una vasta gamma di operazioni di analisi genomica. Usato per intersecare, raggruppare, convertire e contare dati in formato BAM, BED, GFF/GTF, VCF.

## Usage
- `bedtools intersect -a percorso/del/file_A -b percorso/del/file_B1 percorso/del/file_B2 ... -s >percorso/del/file_output` : interseca il file a ed il/i file b in base alla sequenza del filamento s e salva il risultato in un file specifico
- `bedtools intersect -a percorso/del/file1 -b percorso/del/file2 -loj >percorso/del/file_output` : interseca 2 file in base a una left outer join ovvero una unione d'insieme di dati ordinati in colonne che restituisce i dati della tabella di sinistra. Es: riporta ogni proprietà presente nel `file1` e NULL dove non c'è sovrapposizione con `file2`
- `bedtools intersect -a percorso/del/file1 -b percorso/del/file2 -sorted >percorso/del/file_output` : usa un algoritmo più efficiente per intersecare due file precedentemente ordinati
- `bedtools groupby -i percorso/del/file -c 1-3,5 -g 6 -o sum` : seleziona in un file le prime tre colonne e la quinta colonna utilizzando la sesta colonna per raggruppare i dati al fine di poter calcolare tramite un'operazione di addizione la somma delle colonne 1,2,3 e 5 per ciascun gruppo
- `bedtools bamtobed -i percorso/del/file.bam >percorso/del/file.bed` : converti un file in input formattato bam in un file formattato bed
- `bedtools closest -a percorso/del/file1.bed -b percorso/del/file2.bed -d` : trova per tutte le proprietà presenti nel `file1.bed` la più vicina nel `file2.bed` e aggiunge la loro distanza in una ulteriore colonna al risultato finale (i file in input devono essere ordinati)
