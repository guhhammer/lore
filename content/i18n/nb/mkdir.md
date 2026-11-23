# mkdir

Opprett mapper og angi rettighetene deres.

## Usage
- `mkdir sti/til/mappe1 sti/til/mappe2 ...` : opprett bestemte mapper
- `mkdir -p sti/til/mappe1 sti/til/mappe2 ...` : opprett bestemte mapper og deres overordnede mapper ved behov
- `mkdir -m u=rwx,g=rw,o=r sti/til/mappe1 sti/til/mappe2 ...` : opprett mapper med bestemte rettigheter
- `mkdir -p sti/til/{a,b}/{x,y,z}/{h,i,j}` : opprett flere nøstede mapper rekursivt
