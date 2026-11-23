# bwa

Herramienta de alineación Burrows-Wheeler. Mapeador de secuencias de ADN cortas y poco divergentes frente a un gran genoma de referencia, como el genoma humano.

## Usage
- `bwa index ruta/a/referencia.fa` : indexa el genoma de referencia
- `bwa mem -t 32 ruta/a/referencia.fa ruta/a/lectura_solo_extremo.fq.gz | gzip >ruta/a/alineamiento_solo_extremo.sam.gz` : mapea las lecturas de un solo extremo (secuencias) al genoma indexado utilizando 32 subprocesos y comprime el resultado para ahorrar espacio
- `bwa mem -t 32 ruta/a/referencia.fa ruta/a/lectura_par_final_1.fq.gz ruta/a/lectura_par_final_2.fq.gz | gzip >ruta/a/alineamiento_par_final.sam.gz` : mapea las lecturas del par final (secuencias) al genoma indexado usando 32 subprocesos y comprime el resultado para ahorrar espacio
- `bwa mem -M -t 32 ruta/a/referencia.fa ruta/a/lectura_par_final_1.fq.gz ruta/a/lectura_par_final_2.fq.gz | gzip >ruta/a/alineamiento_par_final.sam.gz` : mapea las lecturas del par final (secuencias) al genoma indexado usando 32 subprocesos con Marcadores divididos más cortos como secundarios para la compatibilidad del archivo SAM de salida con el software Picard y luego comprime el resultado
- `bwa mem -C -t 32 ruta/a/referencia.fa ruta/a/lectura_par_final_1.fq.gz ruta/a/lectura_par_final_2.fq.gz | gzip >ruta/a/lectura_par_final.sam.gz` : mapea las lecturas finales del par (secuencias) al genoma indexado usando 32 subprocesos con Comentarios FASTA/Q (p. ej. BC:Z:CGTAC) anexando a un resultado comprimido

## Links
- More information: https://manned.org/bwa
