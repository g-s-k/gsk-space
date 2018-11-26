# gsk-space
my sandbox for trying new webapp-y things

## How to...

### setup build environment

* install [docker](https://docs.docker.com/install/) and [docker-compose](https://docs.docker.com/compose/install/)
* install nodejs and `yarn`
* install [rustup](https://rustup.rs) and the MUSL toolchain
```shell
$ curl -sSf https://sh.rustup.rs | sh

$ rustup toolchain add x86_64-unknown-linux-musl
```

if you also want to deploy remotely:

* install [docker-machine](https://docs.docker.com/machine/install-machine/)
* [create a machine or cluster](https://docs.docker.com/machine/get-started-cloud/)

### develop
```shell
$ eval $(docker-machine env -u)

$ make start # to start the development build

$ make stop # to stop it
```

### deploy
```shell
$ eval $(docker-machine env $MACHINE_NAME)

$ make deploy
```
