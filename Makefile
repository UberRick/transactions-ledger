.PHONY: dev
dev:
	cargo run -- transactions.csv

.PHONY: run
run:
	cargo run -- transactions.csv > accounts.csv

.PHONY: seed
seed:
	./scripts/seed.sh

.PHONY: test
test:
	cargo test

.PHONY: audit
audit:
	cargo audit

.PHONY: clean
clean:
	rm -f transactions.csv accounts.csv

