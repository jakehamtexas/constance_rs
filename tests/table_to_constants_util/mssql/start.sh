#!/bin/bash

pw=MyAwS0M3P4SsW0Rd!
docker run -d -p 1433:1433 -e ACCEPT_EULA=Y -e SA_PASSWORD=$pw --rm --name mssql mcr.microsoft.com/mssql/server:2017-CU12-ubuntu
docker ps -a
docker cp init.sql mssql:/init.sql
docker exec mssql sh -c "/opt/mssql-tools/bin/sqlcmd -H localhost -U sa -P $pw -i ./init.sql"
