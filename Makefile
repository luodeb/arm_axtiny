ARCH ?= aarch64
APP := arm_axtiny
PLAT ?= qemu
OBJDUMP ?= rust-objdump -d --print-imm-hex --x86-asm-syntax=intel
OBJCOPY ?= rust-objcopy --binary-architecture=$(ARCH)

TARGET := aarch64-unknown-none-softfloat

OUT_ELF := $(CURDIR)/target/$(TARGET)/release/$(APP)
OUT_BIN := $(OUT_ELF).bin
OUT_ASM := $(OUT_ELF)_asm.txt
OUT_RASPI := tools/chainloader/demo_payload_rpi4.img

qemu_args-aarch64 := \
  -cpu cortex-a72 \
  -machine virt \
  -kernel $(OUT_BIN)

export AX_PLATFORM=$(PLAT)

features := plat_$(PLAT)

all: build

build:
	cargo build -p $(APP) --target $(TARGET) --release --no-default-features --features $(features)

	@echo "Dump $(OUT_ASM)"
	@rust-objdump -d --print-imm-hex $(OUT_ELF) > $(OUT_ASM)

$(OUT_BIN): build
	$(OBJCOPY) --strip-all -O binary $(OUT_ELF) $(OUT_BIN)

run: $(OUT_BIN)
ifeq ($(PLAT), qemu)
	qemu-system-$(ARCH) $(qemu_args-$(ARCH)) -nographic
else ifeq ($(PLAT), raspi)
	@echo "Copy $(OUT_BIN) to $(OUT_RASPI)"
	@cp $(OUT_BIN) $(OUT_RASPI)
	@echo "Build chainboot"
	@cd tools/chainloader && make chainboot
endif

disasm:
	$(OBJDUMP) $(OUT_ELF) | less

clippy:
	cargo clippy -p $(APP) --target $(TARGET)

clean:
	cargo clean
	@rm -f *.lds
