# Email subscribe API from zero to prod using axum and sqlx

## how to setup:

### local:

```bash
   # run docker images
   bash ./env/test/setup start

   # stop docker images
   bash ./env/test/setup stop

   # run with local configfile
   cargo run -- -f localsetup.toml

```

### production:

```bash
   # setup
   bash ./env/production/setup start
```

## sqlx command:

```bash
    #run sqlx migrate
   sqlx migrate run

   # run sqlx prepare
   cargo sqlx prepare
```

## lint command:

```bash
   # run clippy lint
   cargo clippy -- -D warnings

```

- **add tracing and prometheus**: traces are collected using [Grafana Tempo](https://grafana.com/oss/tempo/) and they can be inspected using [Grafana](https://grafana.com/) at the address `http://localhost:3000`. send request and then can find service in grafana-tempo.

- **hyper**: [hyper](https://crates.io/crates/hyper) is used as HTTP client;

- **config**: hierarchical configuration is not implemented, configuration can be customized using environment variables; database migrations can be executed on service startup.

- **claym**: unmaintained project [claim](https://crates.io/crates/claim) is replaced with [claym](https://crates.io/crates/claym);

- **email**: [hyper](https://crates.io/crates/hyper) is used as HTTP client to send email, HTTP client timeout can be configured using [humantime](https://crates.io/crates/humantime).
