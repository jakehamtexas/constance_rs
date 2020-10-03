#!/bin/bash

# Vars
pw=MyAwS0M3P4SsW0Rd!
port=1433
host=localhost
user=sa

# Setup
(cd pretest && cargo run --release ../values)
docker run -d -p $port:$port -e ACCEPT_EULA=Y -e SA_PASSWORD=$pw --rm --name mssql mcr.microsoft.com/mssql/server:2017-CU12-ubuntu
docker ps -a
docker cp init.sql mssql:/init.sql
docker exec mssql sh -c "/opt/mssql-tools/bin/sqlcmd -H $host -U $user -P $pw -i ./init.sql"

# Run the tests!
RUST_BACKTRACE=1 PORT=$port HOST=$host PASSWORD=$pw USER=$user cargo test table_to_constants_mssql_simple_enum

# Teardown
rm init.sql
docker kill mssql
printf 'y\n' | docker system prune -a