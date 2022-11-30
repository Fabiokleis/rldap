#!/bin/bash
docker image rm "$(docker images --filter "dangling=true" -q --no-trunc)"
docker build . -t fishingboo/rldap-osixia-openldap:latest --no-cache
docker image rm "$(docker images --filter "dangling=true" -q --no-trunc)"
docker push fishingboo/rldap-osixia-openldap:latest
