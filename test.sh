#!/bin/bash

#### Test the local build of rldap on ldap container ####

declare -A LOCAL_ENVS

function load_default() {
    true;
}

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
        i=$((i+1))
    done < "$file"

}

function do_test() {
    SERVER="${LOCAL_ENVS["LDAP_SERVER"]}"

    #Create the first ldap server, save the container id in LDAP_CID and get its IP:
    LDAP_CID1=$(docker run --name "rldap-ldap" --hostname "$SERVER" --detach fishingboo/rldap-osixia-openldap:latest)
    LDAP_IP=$(docker inspect -f "{{ .NetworkSettings.IPAddress }}" "$LDAP_CID1")

    #Add the pair "ip hostname" to /etc/hosts
    docker container exec "$LDAP_CID1" bash -c "echo $LDAP_IP $SERVER >> /etc/hosts"
    LDAP_CID2=$(docker container run --detach --name "rldap-test" fishingboo/rldap:latest)
    clean_containers "$LDAP_CID1" "$LDAP_CID2"
}

function clean_containers() {
    docker container stop "$1" "$2"
    docker container rm "$1" "$2"
}

case $1 in
    ".env") 
        load_from_dotenv "$1"
        do_test
        ;;
    "$PWD/.env")
        load_from_dotenv "$1"
        do_test
        ;;
    "*") load_default ;;
esac


