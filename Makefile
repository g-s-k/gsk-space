.PHONY = build deploy

all: build

start:
	docker-compose -f docker/docker-compose.dev.yml up -d --build

stop:
	docker-compose -f docker/docker-compose.dev.yml down && docker volume prune -f

build:
	cd client; yarn build
	cd server; cargo build --release --target=x86_64-unknown-linux-musl

deploy: build
	docker-compose -f docker/docker-compose.yml up -d --build
