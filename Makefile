# Variables

CONFIG := debug
ifeq ($(RELEASE),1)
    CONFIG := release
endif

# Inputs

KERNEL_ASM := src/kernel.asm
KERNEL_SRC := Cargo.toml $(wildcard **/*.rs)
LINKER_SCRIPT := link.ld
GRUB_CFG := iso/grub.cfg

# Targets

OUTPUT_DIR := target
KERNEL_OBJ := $(OUTPUT_DIR)/kernel.o
KERNEL_LIB := target/x86-myria/$(CONFIG)/libmyria.a
KERNEL := $(OUTPUT_DIR)/kernel.$(CONFIG)

ISO_DIR := $(OUTPUT_DIR)/iso.$(CONFIG)
ISO_BOOT_DIR := $(ISO_DIR)/boot
ISO_GRUB_DIR := $(ISO_BOOT_DIR)/grub
ISO_GRUB_CFG := $(ISO_GRUB_DIR)/grub.cfg
ISO_KERNEL := $(ISO_BOOT_DIR)/kernel
ISO_IMAGE := $(OUTPUT_DIR)/myria-$(CONFIG).iso

# Properties

ifeq ($(CONFIG),release)
    CARGO_FLAGS += --release
endif

# Rules

all: $(ISO_IMAGE)
clean:
	rm -rf $(OUTPUT_DIR)

$(KERNEL_OBJ): $(KERNEL_ASM)
	nasm -f elf32 $(KERNEL_ASM) -o $(KERNEL_OBJ)

$(KERNEL_LIB): $(KERNEL_SRC)
	cargo xbuild $(CARGO_FLAGS)

$(KERNEL): $(KERNEL_OBJ) $(KERNEL_LIB)
	ld -n -m elf_i386 -T $(LINKER_SCRIPT) -o $(KERNEL) $(KERNEL_OBJ) $(KERNEL_LIB)

$(ISO_GRUB_CFG): $(GRUB_CFG)
	mkdir -p $(ISO_GRUB_DIR)
	cp -f $(GRUB_CFG) $(ISO_GRUB_CFG)

$(ISO_KERNEL): $(KERNEL)
	mkdir -p $(ISO_BOOT_DIR)
	ln -f $(KERNEL) $(ISO_KERNEL)

$(ISO_DIR): $(ISO_KERNEL) $(ISO_GRUB_CFG)

$(ISO_IMAGE): $(ISO_DIR)
	grub-mkrescue -d /usr/lib/grub/i386-pc -o $(ISO_IMAGE) $(ISO_DIR)
