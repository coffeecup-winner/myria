# Sources

KERNEL_ASM := src/kernel.asm
LINKER_SCRIPT := link.ld
GRUB_CFG := iso/grub.cfg

# Targets

OUTPUT_DIR := build
KERNEL_OBJ := $(OUTPUT_DIR)/kernel.o
KERNEL := $(OUTPUT_DIR)/kernel

ISO_DIR := $(OUTPUT_DIR)/iso
ISO_BOOT_DIR := $(ISO_DIR)/boot
ISO_GRUB_DIR := $(ISO_BOOT_DIR)/grub
ISO_GRUB_CFG := $(ISO_GRUB_DIR)/grub.cfg
ISO_KERNEL := $(ISO_BOOT_DIR)/kernel
ISO_IMAGE := $(OUTPUT_DIR)/myria.iso

# Rules

all: $(ISO_IMAGE)
clean:
	rm -rf $(OUTPUT_DIR)

$(KERNEL_OBJ):
	nasm -f elf32 $(KERNEL_ASM) -o $(KERNEL_OBJ)

$(KERNEL): $(KERNEL_OBJ)
	ld -n -m elf_i386 -T $(LINKER_SCRIPT) -o $(KERNEL) $(KERNEL_OBJ)

$(ISO_BOOT_DIR):
	mkdir -p $(ISO_BOOT_DIR)

$(ISO_GRUB_DIR):
	mkdir -p $(ISO_GRUB_DIR)

$(ISO_GRUB_CFG): $(ISO_GRUB_DIR)
	cp -f $(GRUB_CFG) $(ISO_GRUB_CFG)

$(ISO_KERNEL): $(ISO_BOOT_DIR) $(KERNEL)
	ln -f $(KERNEL) $(ISO_KERNEL)

$(ISO_DIR): $(ISO_KERNEL) $(ISO_GRUB_CFG)

$(ISO_IMAGE): $(ISO_DIR)
	grub-mkrescue -d /usr/lib/grub/i386-pc -o $(ISO_IMAGE) $(ISO_DIR)
