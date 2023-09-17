# OSCloud

OSCloud is a Home Server all in one software. It aims to provide easy files gestion / share / viewing across the network and will in it's near future implement decentralized files storage (storage across many differents servers without one central entity). The project is still under heavy developpement and far form is objectives.
The Front uses React TS, and the Back uses Rust !

## Front
### Front installation
```
npm install       # install node modules
```

### Front usage
```
npm run dev       # run the dev version     (auto update on file change)
npm run build     # build the app           (not tested yet)
npm run preview   # preview the builded app (not tested yet)
```
## Back
### Back installation

Docker and PostgreSQL is requiered
```
cd Back
./install_dev_env.sh       # Install the requiered tools
# Setup the database with docker
docker run --name oscloud-users -e POSTGRES_PASSWORD=password -e POSTGRES_USER=root -e POSTGRES_DB=oscloud-users -p 5432:5432 -v "$(pwd)"/database.sql:/docker-entrypoint-initdb.d/init.sql -d postgres:alpine

docker start oscloud-users # start the database
disel migration run        # populate the database with the requiered tables
```

### Back usage
```
cargo run                  # run the dev version
cargo make run             # run the dev version with hot reload
cargo run --release        # build the app
npm run preview            # preview the builded app
```

Locust is being implemented to benchmark the back,
the installation is not finished, resources to start it could be found in the Locust folder
## Authors

- [Axel Denis](https://github.com/axel-denis)
- [Arthur Aillet](https://github.com/Arthur-Aillet)

## Contributing

Contributions are always welcome! <3
