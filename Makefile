.PHONY: build

build:
	$(MAKE) -C examples/demo build
	cd runtime && npm run build

.PHONY: clean

clean:
	$(MAKE) -C examples/demo clean

.PHONY: test

test:
	$(MAKE) -C examples/demo test
