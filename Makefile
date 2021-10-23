custom:
	cargo build --release --offline
	./target/release/game_of_life

glider:
	cargo build --release --offline --bin=glider
	./target/release/glider

pulsar:
	cargo build --release --offline --bin=pulsar
	./target/release/pulsar

collision:
	cargo build --release --offline --bin=collision
	./target/release/collision

collision_w:
	cargo build --release --offline --bin=collision_w
	./target/release/collision_w

random:
	cargo build --release --offline --bin=random
	./target/release/random

clean: 
	rm -r target; rm Cargo.lock
