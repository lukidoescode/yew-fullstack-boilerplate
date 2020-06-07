# Backend

This is the backend crate. It is built using [actix-web](https://crates.io/crates/actix-web). The backend operates in two modes:

- it can forward the output of the webpack server for development purposes enabled by the `forward-frontend` feature
- it can serve up the staticly built frontend files, which is its normal mode of operation.

## Environment Variables

- `YEW_FULLSTACK_HOST` the host to listen on. `127.0.0.1` by default. Overwritten in Dockerfile by `0.0.0.0` in order to allow access from the docker host.
- `YEW_FULLSTACK_PORT` the port to listen on. `3000` by default. Overwritten in Dockerfile by `8080`.
- `YEW_FULLSTACK_STATIC` the directory the content to statically serve (the web client) resides in. `/usr/local/share/yew-fullstack/www` by default.
- `YEW_FULLSTACK_DB_CONNSTR` the [MongoDB connection string](https://docs.mongodb.com/manual/reference/connection-string/) for connecting to the database server. Defaults to `mongodb://localhost:27017`.
- `YEW_FULLSTACK_DB_NAME` the database name on the database server. Defaults to `yew-fullstack`.
