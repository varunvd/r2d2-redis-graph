.PHONY: tests
.ONESHELL: tests

tests:
	docker run --name r2d2-redisgraph-rs-tests -d --rm -p 6379:6379 redislabs/redisgraph \
		&& cargo test
	docker stop r2d2-redisgraph-rs-tests
