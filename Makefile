.PHONY = build deploy

all: build

build:
	cd client; yarn build
	cd server; cargo build --release --target=x86_64-unknown-linux-musl

deploy: build
	docker-compose -f docker/docker-compose.yml up -d --build
