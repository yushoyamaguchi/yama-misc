sudo-in-advance:
	sudo pwd

docker-image:
	docker build --network=host -t grpc-bidire ./test-with-clab

client-build:
	cargo build --release --manifest-path ./grpc-clt/Cargo.toml

server-build:
	cargo build --release --manifest-path ./grpc-svr/Cargo.toml

setup: sudo-in-advance docker-image client-build server-build
	sudo containerlab -t test-with-clab/lab.yaml deploy

destroy:
	sudo containerlab -t test-with-clab/lab.yaml destroy