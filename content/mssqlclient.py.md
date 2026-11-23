# mssqlclient.py

Connect to Microsoft SQL Server instances and execute queries. Part of the Impacket suite.

## Usage
- `mssqlclient.py -windows-auth domain/username:password@target` : connect to an MSSQL server using Windows authentication
- `mssqlclient.py username:password@target` : connect using SQL server authentication
- `mssqlclient.py domain/username@target -hashes LM_Hash:NT_Hash` : connect using pass-the-hash authentication
- `mssqlclient.py -k domain/username@target` : connect using Kerberos authentication (requires valid tickets)
- `mssqlclient.py username:password@target -query "SELECT user_name();"` : execute a specific SQL command upon connection
- `mssqlclient.py username:password@target -file path/to/sql_file.sql` : execute multiple SQL commands from a file
- `mssqlclient.py username:password@target -db database_name` : connect to a specific database instance (default is `None`)
- `mssqlclient.py username:password@target -show` : display SQL queries before execution

## Categories
databases, users, files

## Links
- More information: https://github.com/fortra/impacket
- tldr page: https://tldr.inbrowser.app/pages/common/mssqlclient.py
