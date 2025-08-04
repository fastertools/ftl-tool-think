.PHONY: build clean test check format lint serve

build:
	cd think && make build

clean:
	cd think && make clean

test:
	cd think && make test

check:
	cd think && make check

format:
	cd think && make format

lint:
	cd think && make lint

serve: build
	ftl serve

dev: format lint test
	@echo "Development checks passed!"