# ksox-template

The KSOX Project

#### `./k8s/patches/dev/envs/config.env`:

```
SURREAL_BIND=0.0.0.0:80
```

#### `./k8s/patches/dev/envs/secrets.env`:

```
SURREAL_USER=surrealuser
SURREAL_PASS=surrealp4ssword
```

#### `./.cargo/config.toml`:

```
[env]
KSOX_SERVER_SURREALDB_URL = "http://surrealdb.test/"
KSOX_SERVER_REDIS_URL = "redis://redis.test/"
KSOX_SERVER_API_BIND = "0.0.0.0:8080"
```
