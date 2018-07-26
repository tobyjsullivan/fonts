SHELL := /bin/bash

.PHONE: test

test:
	cargo test
	cd font && cargo test

