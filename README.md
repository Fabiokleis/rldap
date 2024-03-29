# Rldap
using ldap bindings for rust, connecting on ldap server with docker containers envinronment

### Build statically with musl
```bash
cargo build --target x86_64-unknown-linux-musl --release
```

### Run release
```bash
cargo run --target x86_64-unknown-linux-musl --release
```

### Pull docker images
[rldap](https://hub.docker.com/repository/docker/fishingboo/rldap) are the application image

[rldap-osixia-ldap](https://hub.docker.com/repository/docker/fishingboo/rldap-osixia-openldap) are the ldap image
```bash
docker image pull fishingboo/rldap:latest
```
```bash
docker image pull fishingboo/rldap-osixia-openldap:latest
```

### Container .env configurations setup
```.env
LDAP_DOMAIN=jacaranda.local
LDAP_SERVER=rldap.jacaranda.local
LDAP_BASE_DN=dc=jacaranda,dc=local
LDAP_BIND_DN=cn=admin,dc=jacaranda,dc=local
LDAP_ADMIN_PASSWORD=Mypasswd@123
```

### Test
Use the shell script test.sh to test rldap by creating two containers.
One openldap server and other the rldap container that makes the ldap request.
All connections by default are using ldaps in 636 port.

the openldap container that I used its [docker-openldap](https://github.com/osixia/docker-openldap)
I extend for my case following the [advanced-user-guide](https://github.com/osixia/docker-openldap#advanced-user-guide)

you need the shell scripts to test, ensure that they have permission
```bash
chmod +x test.sh; chmod +x ldap_build/build.sh
```

#### Make
use the Makefile to build, up, and test containers
```bash
make # run build_ldap, build, up and test
```
```bash
make build # cleanups dangling images and build rldap image
```
```bash
make build_ldap # cleanups dangling images and build rldap-ldap image
```
```bash
make up # after builds steps, up containers rldap-ldap rldap-test
```
```bash
make test # after container start, make rldap-test request to rldap-ldap container
```
```bash
make update # builds rldap to linux-musl in --release target, copy rldap bin and .env to running rldap-test container 
```
```bash
make clean # cleanups dangling images, stop containers and remove
```
