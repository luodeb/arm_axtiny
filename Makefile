ARCH ?= aarch64
APP := arm_axtiny
PLAT ?= qemu
OBJDUMP ?= rust-objdump -d --print-imm-hex --x86-asm-syntax=intel
OBJCOPY ?= rust-objcopy --binary-architecture=$(ARCH)

TARGET := aarch64-unknown-none-softfloat

OUT_ELF := $(CURDIR)/target/$(TARGET)/release/$(APP)
OUT_BIN := $(OUT_ELF).bin
OUT_ASM := $(OUT_ELF)_asm.txt

qemu_args-aarch64 := \
  -cpu cortex-a72 \
  -machine raspi4b \
  -kernel $(OUT_BIN)

all: build

build:
	cargo build -p $(APP) --target $(TARGET) --release --no-default-features 

	@echo "Dump $(OUT_ASM)"
	@rust-objdump -d --print-imm-hex $(OUT_ELF) > $(OUT_ASM)

$(OUT_BIN): build
	$(OBJCOPY) --strip-all -O binary $(OUT_ELF) $(OUT_BIN)

run: $(OUT_BIN)
	qemu-system-$(ARCH) $(qemu_args-$(ARCH)) -nographic

disasm:
	$(OBJDUMP) $(OUT_ELF) | less

clippy:
	cargo clippy -p $(APP) --target $(TARGET)

clean:
	cargo clean
