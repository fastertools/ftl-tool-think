.PHONY: build clean test check format lint serve

build:
	ftl build	

clean:
	make -C think clean

test:
	make -C think test

check:
	make -C think check

format:
	make -C think format

lint:
	make -C think lint

serve: build
	ftl up

dev: format lint test
	@echo "Development checks passed!"