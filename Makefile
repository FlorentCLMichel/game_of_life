glider:
	cargo build --release --offline --bin=glider
	./target/release/glider

pulsar:
	cargo build --release --offline --bin=pulsar
	./target/release/pulsar

collision:
	cargo build --release --offline --bin=collision
	./target/release/collision

clean: 
	rm -r target; rm Cargo.lock
