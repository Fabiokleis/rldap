FROM osixia/openldap:latest
MAINTAINER Fabio Kleis <fabiohkrc@gmail.com>

ADD bootstrap /container/service/slapd/assets/config/bootstrap
ADD certs /container/service/slapd/assets/certs
ADD environment /container/environment/01-custom
# 389 636
EXPOSE 636
