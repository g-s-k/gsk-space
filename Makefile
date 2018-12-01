.PHONY = build deploy start stop

all: build

start:
	cd docker; docker-compose up -d --build

stop:
	cd docker; docker-compose down

build:
	cd docker; DOCKER_TARGET=prod docker-compose build

push: build
	cd docker; docker-compose push

deploy:
	cd docker; docker-compose -f docker-compose.yml -f docker-compose.prod.yml pull
	cd docker; docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d

undeploy:
	cd docker; docker-compose -f docker-compose.yml -f docker-compose.prod.yml down
