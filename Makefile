.PHONY = build deploy start stop

export DOCKER_BUILDKIT=1

all: build

start:
	docker-compose -f docker/docker-compose.dev.yml up -d --build

stop:
	docker-compose -f docker/docker-compose.dev.yml down && docker volume prune -f

build:
	cd client; yarn build

deploy: build
	docker-compose -f docker/docker-compose.yml up -d --build
