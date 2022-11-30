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

### test
Use the script test.sh to test rldap by creating two containers
one the openldap server and other the rldap container that makes the ldap request.
First build docker images with docker build, the rldap Dockerfile its in 
the root of the project, the openldap Dockerfile its in ldap_test directory.
if you will use the shell scripts, ensure that they have permission
```bash
chmod +x test.sh; chmod +x ldap_test/build.sh
```

```bash
docker build . -t fishingboo/rldap:latest
cd ldap_test && ./build.sh
```

```bash
./test.sh 
```
