
#if ARCH == aarch64
#undef __386__
#undef __x86_64__
#define __aarch64__
#define CONFIG_ARM_64
#endif

#include <xen/types.h>
#include <public/xen.h>
#include <public/sched.h>
#include <public/elfnote.h>
