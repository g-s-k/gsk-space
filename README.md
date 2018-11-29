# gsk-space
my sandbox for trying new webapp-y things

## How to...

### setup build environment

* install [docker](https://docs.docker.com/install/) and [docker-compose](https://docs.docker.com/compose/install/)

if you also want to deploy remotely:

* install [docker-machine](https://docs.docker.com/machine/install-machine/)
* [create a machine or cluster](https://docs.docker.com/machine/get-started-cloud/)

### develop
```shell
# if you use docker-machine
$ eval $(docker-machine env -u)

# to start the development build
$ make start

# to stop it
$ make stop
```

### deploy
```shell
$ eval $(docker-machine env $MACHINE_NAME)

$ make deploy
```
