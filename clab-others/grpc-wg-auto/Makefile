sudo-in-advance:
	sudo pwd

docker-image:
	docker build --network=host -t grpc-wg-auto ./test-with-clab

client-build:
	cargo build --release --manifest-path ./grpc-clt/Cargo.toml

client-clean:
	cargo clean --manifest-path ./grpc-clt/Cargo.toml

server-build:
	cargo build --release --manifest-path ./grpc-svr/Cargo.toml

server-clean:
	cargo clean --manifest-path ./grpc-svr/Cargo.toml

client-test:
	cargo test --release --manifest-path ./grpc-clt/Cargo.toml

setup: sudo-in-advance docker-image client-build server-build
	sudo containerlab -t test-with-clab/lab.yaml deploy

destroy:
	sudo containerlab -t test-with-clab/lab.yaml destroy