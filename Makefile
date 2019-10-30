.mkdir:
	mkdir -p $(HOME)/.shortcut-cli

.run-test:
	cargo test -- --nocapture

all: .mkdir .run-test
	cargo build --release

debug: .mkdir .run-test
	cargo build

install:
	cp ./target/release/shortcut-cli /usr/bin
	chmod +x /usr/bin/shortcut-cli

linux-package:
	cargo deb
	cargo rpm build

darwin-package:
	scripts/darwin-package.sh

clean:
	rm -rf target
