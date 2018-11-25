.PHONY = build deploy

all: build

build:
	cd client; yarn build

deploy: build
	docker-compose up -d --build
