#!/bin/bash
docker build . -t fishingboo/rldap-osixia-openldap:latest --no-cache
docker push fishingboo/rldap-osixia-openldap:latest
