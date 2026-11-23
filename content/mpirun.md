# mpirun

Execute serial and parallel jobs in Open MPI.

## Usage
- `mpirun path/to/executable` : execute an Open MPI program
- `mpirun -n n path/to/executable` : execute an Open MPI program with `n` parallel processes
- `mpirun -oversubscribe path/to/executable` : allow more processes than available physical cores

## See also
- mpic++

## Categories
processes, files

## Links
- More information: https://docs.open-mpi.org/en/main/man-openmpi/man1/mpirun.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/mpirun
