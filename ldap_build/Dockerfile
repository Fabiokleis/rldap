FROM osixia/openldap:1.5.0
MAINTAINER Fabio Kleis <fabiohkrc@gmail.com>

ADD bootstrap /container/service/slapd/assets/config/bootstrap
ADD certs /container/service/slapd/assets/certs
ADD environment /container/environment/01-custom
EXPOSE 389 636
