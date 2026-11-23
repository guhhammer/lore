# dbiprof

Command-line client for DBI::ProfileData.

## Usage
- `See a report of the ten queries with the longest total runtime in the profile dump file prof1.out:` : the general form, as the manual gives it
- `dbiprof prof1.out See the top 10 most frequently run queries in the profile file dbi.prof (the default):` : another form of the same command
- `dbiprof --sort count See the same report with 15 entries:` : another form of the same command
- `dbiprof --sort count --number 15` : another form of the same command
- `dbiprof --number N` : produce this many items in the report
- `dbiprof --sort field` : sort results by the given field
- `dbiprof --reverse` : reverses the selected sort

## Categories
text
