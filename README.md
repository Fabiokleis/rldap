# rldap
using ldap bindings for rust

### build statically with musl
```bash
cargo build --target x86_64-unknown-linux-musl --release
```

### run release
```bash
cargo run --target x86_64-unknown-linux-musl --release
```

### build run and remove container
```bash
docker build . -t fishingboo/rldap:latest
docker container run -it --name rldap-test fishingboo/rldap:latest
docker container rm rldap-test
```
