
DROP DATABASE IF EXISTS rust_warp_postgres;
DROP USER IF EXISTS rust_warp_postgres_user;

CREATE USER rust_warp_postgres_user WITH PASSWORD 'password';
CREATE DATABASE rust_warp_postgres OWNER rust_warp_postgres_user ENCODING = 'UTF-8';

