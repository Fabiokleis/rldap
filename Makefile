NAME = rldap
VERSION = 0.1.0


all: build_ldap build up test
PHONY: all

build:
	./test.sh -c -b -c

up:
	./test.sh -e=.env -u=rldap-ldap -u=rldap-test

test:
	@sleep 1
	./test.sh -t

build_ldap:
	cd ldap_build && ./build.sh

update:
	./test.sh --update

clean:
	./test.sh -c 
