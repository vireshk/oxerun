
ARCH ?= x86_64
BINDINGS := xen-sys/src/$(ARCH)/bindgen.rs
XEN_PATH ?= ../xen
XEN_INCLUDE = $(XEN_PATH)/xen/include
BINDGEN := bindgen

all: $(BINDINGS)

xen-sys/src/$(ARCH)/bindgen.rs: $(XEN_INCLUDE)/public/xen.h xen-sys/wrapper.h
	@mkdir -p $(@D)
	$(BINDGEN) \
		--rust-target nightly \
		--use-core \
		--ctypes-prefix cty \
		-o $@ \
		xen-sys/wrapper.h \
		-- -I$(XEN_INCLUDE)
