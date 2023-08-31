**/!\\ Add sudo if process don't work, possible fix /!\\**

- create it:
```bash
docker run --name oscloud-users -e POSTGRES_PASSWORD=password -e POSTGRES_USER=root -e POSTGRES_DB=oscloud-users -p 5432:5432 -v "$(pwd)"/database.sql:/docker-entrypoint-initdb.d/init.sql -d postgres:alpine
```
```
disel migration run
```
- start it:
```bash
docker start oscloud-users
```

- stop it:
```bash
sudo ss -lptn 'sport = :5432'
```

```bash
docker stop oscloud-users
```

- delete it:
```bash
docker rm oscloud-users
```

- see it:
```bash
docker ps -a
```

- look in it:
```bash
docker exec -it oscloud-users bash
```

```bash
psql -d postgres -U root
```
```bash
\l  #to list databases
\dt #to list tables
```

- add table

```bash
diesel migration run
```

- refresh with new table

```bash
diesel migration redo
```
