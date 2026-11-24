# qsub

Submit a script to the queue management system TORQUE.

## Usage
- `qsub script.sh` : submit a script with default settings (depends on TORQUE settings)
- `qsub -l walltime=1:2:3 script.sh` : submit a script with a specified wallclock runtime limit of 1 hour 2 minutes and 3 seconds
- `qsub -l nodes=2:ppn=4 script.sh` : submit a script that is executed on 2 nodes using 4 cores per node
- `qsub -q queue_name script.sh` : submit a script to a specific queue. Note that different queues can have different maximum and minimum runtime limits

## Categories
development

## Links
- More information: https://manned.org/qsub.1
- tldr page: https://tldr.inbrowser.app/pages/linux/qsub
- Wikipedia: https://en.wikipedia.org/wiki/Qsub
