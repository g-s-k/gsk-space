# gsk-space
my sandbox for trying new webapp-y things

## How to...

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
