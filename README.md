

# KSOX-SERVER

[![GitHub Workflow Status Rust Build](https://github.com/visoftsolutions/ksox-server/actions/workflows/master_build.yml/badge.svg)](https://github.com/visoftsolutions/ksox-server/actions/workflows/master_build.yml)
![Version](https://img.shields.io/badge/version-0.1.0-blue)

## 🚀 Quick Start Guide

Embarking on your journey with the KSOX-SERVER workspace? Here's a streamlined guide:

### 1. **Cleanup**

Begin with a clean slate by removing residual files from any prior builds:

```sh
cargo clean
```

### 2. **Build**

Initialize your environment by installing all necessary dependencies:

```sh
cargo build
```

### 3. **Code Styling**

Ensure your code exudes clarity and consistency with our predefined standards:

```sh
cargo fmt
cargo sort -w
```

### 4. **Linting**

Maintain the integrity and quality of your code with our linting tools:

```sh
cargo clippy
```

### 5. **Unused Dependencies**

Regularly check for unused dependencies:

```sh
cargo +nightly udeps
```

## 🤝 Contributing

Encountered a hurdle or have a suggestion? Raise an issue or connect with our dedicated team. Your journey with KSOX-WEB is valued!

### 6. **Enviroment variables**

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
KSOX_SERVER_JWT_SECRET = ""
```