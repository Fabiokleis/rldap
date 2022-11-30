#!/bin/bash
docker build . -t fishingboo/rldap-osixia-openldap:latest
docker push fishingboo/rldap-osixia-openldap:latest
