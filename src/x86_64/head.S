#include <xtf/asm_macros.h>

#include <arch/page.h>

#include <xen/elfnote.h>

/* Guest name and version */
ELFNOTE(Xen, XEN_ELFNOTE_GUEST_OS, .asciz "XTF")
ELFNOTE(Xen, XEN_ELFNOTE_GUEST_VERSION, .asciz "0")

/* PV loader */
ELFNOTE(Xen, XEN_ELFNOTE_LOADER, .asciz "generic")
ELFNOTE(Xen, XEN_ELFNOTE_HYPERCALL_PAGE, _WORD HYPERCALL_PAGE)

/* Xen ABI information */
ELFNOTE(Xen, XEN_ELFNOTE_XEN_VERSION, .asciz "xen-3.0")
ELFNOTE(Xen, XEN_ELFNOTE_FEATURES, .asciz "!writable_page_tables|pae_pgdir_above_4gb")
ELFNOTE(Xen, XEN_ELFNOTE_PAE_MODE, .asciz "yes")

        .text
#ifdef __x86_64__
        .code64
#else
        .code32
#endif

 /* PV entry point. */
GLOBAL(_start)

        /* Move onto our own stack. */
        mov $STACK_TOP, %_ASM_BP
        mov %_ASM_BP, %_ASM_SP

        /* Put the start_info_t pointer from domain build
         * into RDI to be the first arg in rust_entry
         */
        mov %_ASM_SI, %_ASM_DI

        call rust_entry

        /* poweroff() if rust_entry manages to return. */
        call poweroff
ENDFUNC(_start)

/*
 * Local variables:
 * tab-width: 8
 * indent-tabs-mode: nil
 * End:
 */
