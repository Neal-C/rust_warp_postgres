## I AM NOT THE AUTHOR OF THIS CODE;

### code may have been copied as such or received my personal modifications.

### All done with a learning intent, copying means coding while following a tutorial.

## DB

```sh
# Start the database
# Can't map 5432:5432 because I have postgres locally on my machine already on this port
docker run --rm -p 5555:5432 -e "POSTGRES_PASSWORD=postgres" --name rust_warp_postgres postgres:14

# optional psql (other terminal)

docker exec -it -u postgres rust_warp_postgres psql
```

## Development test

```sh
# -q for quiet , -c for clear, -w for what to watch , -x for execute following command
cargo watch -q -c -w src/ -x '--test model_database_ --test-threads=1 --no-capture'
```

## Development Web

```sh
cargo watch -q -c -w src/ -x 'run -- --../frontend/web-folder'
```
