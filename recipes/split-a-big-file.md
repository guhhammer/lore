# split a big file

Cut a big file into smaller pieces, by lines or by size, and join them again.

## Steps
- `split -l 1000 big.txt part_` : pieces of 1,000 lines: part_aa, part_ab…
- `split -b 100M big.bin part_` : pieces of 100 MB
- `split -n 4 big.txt part_` : four pieces of equal size (GNU)
- `split -d -l 1000 --additional-suffix=.csv big.csv part_` : numbered pieces that keep the .csv ending (GNU)
- `cat part_* > big.txt` : join them again

## Notes
- A CSV's header line goes only into the first piece.

## See also
- split
- cat
- csplit

## Categories
files text
