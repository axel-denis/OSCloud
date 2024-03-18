**/!\\ Add sudo if process don't work, possible fix /!\\**

- init diesel
```
diesel migration run
```
- start it:
```bash
docker compose up
```

- stop it:
```bash
docker compose stop
```

- remove it:
```bash
docker compose down
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
