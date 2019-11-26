SOURCES = $(wildcard **/*.rs) $(wildcard **/*.S) $(wildcard **/*.ld)
CARGO_OUTPUT = target/aarch64-unknown-none-softfloat/release/raspi-os
OUTPUT = kernel8.img
TARGET = aarch64-unknown-none-softfloat

.PHONY: all doc qemu clippy clean readelf objdump nm

all: clean $(OUTPUT)

$(OUTPUT): $(CARGO_OUTPUT)
	cp $< .
	cargo objcopy \
		-- \
		--strip-all \
		-O binary $< $(OUTPUT)

$(CARGO_OUTPUT): $(SOURCES)
	RUSTFLAGS="-C link-arg=-Tlink.ld -C target-cpu=cortex-a53 -D warnings" cargo xrustc \
		--target=$(TARGET) \
		--release

doc:
	cargo xdoc --target=$(TARGET) --document-private-items
	xdg-open target/$(TARGET)/doc/raspi-os/index.html

clippy:
	cargo xclippy --target=$(TARGET)

clean:
	rm -rf target

readelf:
	readelf -a raspi-os

objdump:
	cargo objdump --target $(TARGET) -- -disassemble -print-imm-hex raspi-os

nm:
	cargo nm --target $(TARGET) -- raspi-os | sort

run:
	qemu-system-aarch64 -M raspi3 -kernel kernel8.img -d in_asm
