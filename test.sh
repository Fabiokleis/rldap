#!/bin/bash

#### Test the local build of rldap on ldap container ####

declare -A LOCAL_ENVS

function load_from_dotenv() {
    file="$1"
    if [ ! -f "$file" ]; then
        echo "File not found!"
        exit 1
    fi

    while read -r line; do
        key=$(echo "$line" | sed -e '/^#/d;/^\s*$/d' -e "s/'/'\\\''/g" -e "s/=\(.*\)/='\1'/g" | cut -d"'" -f1 | tr -d '=')
        value=$(echo "$line" | sed -e '/^#/d;/^\s*$/d' -e "s/'/'\\\''/g" -e "s/=\(.*\)/='\1'/g" | cut -d"'" -f2)
        echo "$key = $value"
        LOCAL_ENVS["$key"]="$value"
    done < "$file"
}

function up_ldap_container() {
    SERVER="${LOCAL_ENVS["LDAP_SERVER"]}"

    # Create the first ldap server, save the container id in LDAP_CID and get its IP:
    LDAP_CID1=$(docker container run --name "rldap-ldap" --hostname "$SERVER" --detach fishingboo/rldap-osixia-openldap:latest)
    LDAP_IP=$(docker inspect -f "{{ .NetworkSettings.IPAddress }}" "$LDAP_CID1")

    # Add the pair "ip hostname" to /etc/hosts
    docker container exec "$LDAP_CID1" bash -c "echo $LDAP_IP $SERVER >> /etc/hosts"
}

function up_rldap_container() {
    SERVER="${LOCAL_ENVS["LDAP_SERVER"]}"
    LDAP_IP=$(docker inspect -f "{{ .NetworkSettings.IPAddress }}" "rldap-ldap")

    LDAP_CID2=$(docker container run --detach --name "rldap-test" fishingboo/rldap:latest)
    docker container exec "$LDAP_CID2" sh -c "echo $LDAP_IP $SERVER >> /etc/hosts"
}

function build_rldap() {
    cargo build --target x86_64-unknown-linux-musl --release
    docker build . -t fishingboo/rldap:latest --no-cache
    docker push fishingboo/rldap:latest
}

function call_rldap_bin() {
    docker container exec "rldap-test" sh -c "./rldap"
}

function clean_containers() {
    docker container stop "$1" "$2"
    docker container rm "$1" "$2"
    # remove dangling images
    docker image rm "$(docker images --filter "dangling=true" -q --no-trunc)"
}

function show_help() {
    printf '\nthe order matters!\n\n'
    echo 'full usage: ./test.sh --build --env=file --up-container=rldap-ldap --up-container=rldap-test --test --clean'
    echo 'full usage: ./test.sh -b -e=file -u=rldap-ldap -u=rldap-test -t -c'
    printf '\n'

    echo '-b|--build'
    echo 'build the rldap image from local Dockerfile'
    printf '\n'

    echo '-e|--env'
    echo 'load the environment file to up containers'
    printf '\n'

    echo '-u=rldap-ldap|--up-container=rldap-test'
    echo 'up containers to test'
    printf '\n'

    echo '-t|--test'
    echo 'run the bin rldap of rldap-test container'
    printf '\n'

    echo '-c|--clean'
    echo 'stop and remove containers'
}

function run_opts() {
    for i in "$@"; do
        case $i in
           -b|--build)
                build_rldap
                shift
            ;;
            -e=*|--env=*)
                FILE="${i#*=}"
                load_from_dotenv "$FILE"
                shift
            ;;
            -u=rldap-ldap|--up-container=rldap-ldap)
                up_ldap_container
                shift
            ;;
            -u=rldap-test|--up-container=rldap-test)
                up_rldap_container
                shift
            ;;
           -t|--test)
                call_rldap_bin
                shift
            ;;
            -c|--clean)
                clean_containers "rldap-test" "rldap-ldap"
                shift
            ;;
            -h|--help)
                show_help
                shift
            ;;
            -*|*)
                echo "Unknown option $i"
                echo "run ./test.sh -h for help!"
                exit 1
            ;;
        esac
    done
}

if [ "$#" -eq 0 ]; then
    echo "Pass arguments to run tests!"
    echo "run ./test.sh -h for help!"
else
    run_opts "$@"
fi

