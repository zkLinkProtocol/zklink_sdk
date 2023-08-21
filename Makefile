
lint:
	cargo fmt
	cargo clippy
	cargo sort
	bash -c "cd ./common && cargo sort"
	bash -c "cd ./crypto && cargo sort"
	bash -c "cd ./interface && cargo sort"
	bash -c "cd ./types && cargo sort"
	bash -c "cd ./utils && cargo sort"
	cargo machete

lint-check:
	cargo fmt -- --check
	cargo clippy
	cargo sort --check
	bash -c "cd ./common && cargo sort --check"
	bash -c "cd ./crypto && cargo sort --check"
	bash -c "cd ./interface && cargo sort --check"
	bash -c "cd ./types && cargo sort --check"
	bash -c "cd ./utils && cargo sort --check"
	cargo machete

build:
	cargo build

tool:
	cargo install taplo-cli --locked
	cargo install cargo-sort cargo-machete