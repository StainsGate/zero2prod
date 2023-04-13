# Email subscribe API from zero to prod using axum and sqlx

## how to setup:

```bash
   # run docker images
   $  bash ./env/test/setup start

   #run sqlx migrate
   $ sqlx migrate run
```

- **add tracing and prometheus**: traces are collected using [Grafana Tempo](https://grafana.com/oss/tempo/) and they can be inspected using [Grafana](https://grafana.com/) at the address `http://localhost:3000`. send request and then can find service in grafana-tempo.
