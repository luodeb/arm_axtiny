ARCH ?= aarch64
APP := arm_axtiny

OBJDUMP ?= rust-objdump -d --print-imm-hex --x86-asm-syntax=intel
OBJCOPY ?= rust-objcopy --binary-architecture=$(ARCH)

TARGET := aarch64-unknown-none-softfloat
PLAT ?= qemu

OUT_ELF := $(CURDIR)/target/$(TARGET)/release/$(APP)
OUT_BIN := $(OUT_ELF).bin
OUT_RASPI := tools/chainloader/demo_payload_rpi4.img

features := $(PLAT)

qemu_args-aarch64 := \
  -cpu cortex-a72 \
  -machine virt \
  -kernel $(OUT_BIN)

all: build

build:
	cargo build -p $(APP) --target $(TARGET) --release --no-default-features --features "$(features)"

$(OUT_BIN): build
	$(OBJCOPY) --strip-all -O binary $(OUT_ELF) $(OUT_BIN)

run: $(OUT_BIN)
	qemu-system-$(ARCH) $(qemu_args-$(ARCH)) -nographic

raspi: $(OUT_BIN)
	@echo "Copy $(OUT_BIN) to $(OUT_RASPI)"
	@cp $(OUT_BIN) $(OUT_RASPI)
	@echo "Build chainboot"
	@cd tools/chainloader && make chainboot

disasm:
	$(OBJDUMP) $(OUT_ELF) | less

clippy:
	cargo clippy -p $(APP) --target $(TARGET)

clean:
	cargo clean
	@rm -f *.lds
