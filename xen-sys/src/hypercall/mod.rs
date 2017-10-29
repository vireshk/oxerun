/*
 * Copyright 2016-2017 Doug Goldstein <cardoe@cardoe.com>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use self::x86_64::*;

type Hypercall = u64;
const HYPERVISOR_set_trap_table: Hypercall = 0;
const HYPERVISOR_mmu_update: Hypercall = 1;
const HYPERVISOR_set_gdt: Hypercall = 2;
const HYPERVISOR_stack_switch: Hypercall = 3;
const HYPERVISOR_set_callbacks: Hypercall = 4;
const HYPERVISOR_fpu_taskswitch: Hypercall = 5;
const HYPERVISOR_sched_op_compat: Hypercall = 6;
const HYPERVISOR_platform_op: Hypercall = 7;
const HYPERVISOR_set_debugreg: Hypercall = 8;
const HYPERVISOR_get_debugreg: Hypercall = 9;
const HYPERVISOR_update_descriptor: Hypercall = 10;
const HYPERVISOR_memory_op: Hypercall = 12;
const HYPERVISOR_multicall: Hypercall = 13;
const HYPERVISOR_update_va_mapping: Hypercall = 14;
const HYPERVISOR_set_timer_op: Hypercall = 15;
const HYPERVISOR_event_channel_op_compat: Hypercall = 16;
const HYPERVISOR_xen_version: Hypercall = 17;
/// __HYPERVISOR_console_io
const HYPERVISOR_console_io: Hypercall = 18;
const HYPERVISOR_physdev_op_compat: Hypercall = 19;
const HYPERVISOR_grant_table_op: Hypercall = 20;
const HYPERVISOR_vm_assist: Hypercall = 21;
const HYPERVISOR_update_va_mapping_otherdomain: Hypercall = 22;
const HYPERVISOR_iret: Hypercall = 23;
const HYPERVISOR_vcpu_op: Hypercall = 24;
const HYPERVISOR_set_segment_base: Hypercall = 25;
const HYPERVISOR_mmuext_op: Hypercall = 26;
const HYPERVISOR_xsm_op: Hypercall = 27;
const HYPERVISOR_nmi_op: Hypercall = 28;
const HYPERVISOR_sched_op: Hypercall = 29;
const HYPERVISOR_callback_op: Hypercall = 30;
const HYPERVISOR_xenopref_op: Hypercall = 31;
const HYPERVISOR_event_channel_op: Hypercall = 32;
const HYPERVISOR_physdev_op: Hypercall = 33;
const HYPERVISOR_hvm_op: Hypercall = 34;
const HYPERVISOR_sysctl: Hypercall = 35;
const HYPERVISOR_domctl: Hypercall = 36;
const HYPERVISOR_kexec_op: Hypercall = 37;
const HYPERVISOR_tmem_op: Hypercall = 38;
const HYPERVISOR_xc_reserved_op: Hypercall = 39;
const HYPERVISOR_xenpmu_op: Hypercall = 40;
const HYPERVISOR_arch_0: Hypercall = 48;
const HYPERVISOR_arch_1: Hypercall = 49;
const HYPERVISOR_arch_2: Hypercall = 50;
const HYPERVISOR_arch_3: Hypercall = 51;
const HYPERVISOR_arch_4: Hypercall = 52;
const HYPERVISOR_arch_5: Hypercall = 53;
const HYPERVISOR_arch_6: Hypercall = 54;
const HYPERVISOR_arch_7: Hypercall = 55;

pub mod console_io {
    use super::hypercall_3;

    #[derive(Debug)]
    pub enum Command {
        /// CONSOLEIO_write
        Write = 0,
    }

    pub fn write(out: &[u8]) {
        unsafe {
        hypercall_3(super::HYPERVISOR_console_io,
                    Command::Write as u64,
                    out.len() as u64,
                    out.as_ptr() as u64)
        };
    }
}
