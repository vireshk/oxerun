
ARCH := x86_64
TARGET := $(ARCH)-xen-none
CRATE := xen-rust
CRATE_OBJ := $(subst -,_,$(CRATE))
CRATE_BUILD := target/$(TARGET)/debug/lib$(CRATE_OBJ).a
KERNEL := build/$(CRATE)-$(ARCH).bin

LDS := xen-rust.ld
RUST_SRCS := $(wildcard src/**.rs)
SRCS := $(wildcard src/$(ARCH)/*.S)
OBJS := $(patsubst src/$(ARCH)/%.S, build/$(ARCH)/%.o, $(SRCS))

COMMON_FLAGS := -pipe -Iinclude -Isrc/$(ARCH)/include
AFLAGS := $(COMMON_FLAGS) -D__ASSEMBLY__

all: $(KERNEL)

clean:
	@cargo clean
	@rm -rf build

$(KERNEL): $(CRATE_BUILD) $(LDS) $(OBJS)
	ld -n --gc-sections -T $(LDS) -o $(KERNEL) $(OBJS) $(CRATE_BUILD)

$(CRATE_BUILD): $(RUST_SRCS)
	@xargo build --target $(TARGET)

build/$(ARCH)/%.o: src/$(ARCH)/%.S
	@mkdir -p $(@D)
	$(CC) -c -o $@ $(AFLAGS) $<
