# powerstat

Measure the power consumption of a computer that has a battery power source or supports the RAPL interface.

## Usage
- `powerstat` : measure power with the default of 10 samples with an interval of 10 seconds
- `powerstat interval number_of_samples` : measure power with custom number of samples and interval duration
- `powerstat -R interval number_of_samples` : measure power using Intel's RAPL interface
- `powerstat -H interval number_of_samples` : show a histogram of the power measurements
- `powerstat -a interval number_of_samples` : enable all statistics gathering options

## Categories
network, hardware

## Links
- More information: https://manned.org/powerstat
- tldr page: https://tldr.inbrowser.app/pages/linux/powerstat
