# mpirun

Voer seriële en parallelle taken uit in Open MPI.

## Usage
- `mpirun pad/naar/executable` : voer een Open MPI programma uit
- `mpirun -n n pad/naar/executable` : voer een Open MPI programma uit met `n` parallelle processen
- `mpirun -oversubscribe pad/naar/executable` : sta meer processen toe dan beschikbare fysieke cores

## Links
- More information: https://docs.open-mpi.org/en/main/man-openmpi/man1/mpirun.1.html
