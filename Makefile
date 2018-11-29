.PHONY = build deploy start stop

all: build

start:
	docker-compose -f docker/docker-compose.dev.yml up -d --build

stop:
	docker-compose -f docker/docker-compose.dev.yml down && docker volume prune -f

build:
	docker-compose -f docker/docker-compose.yml build

deploy: build
	docker-compose -f docker/docker-compose.yml up -d --build
