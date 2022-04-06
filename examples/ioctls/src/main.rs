/*
 * Copyright 2022-23 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use xen_ioctls::sysctl_types::*;
use xen_ioctls::privcmd::PrivCmdHypercall;
use xen_ioctls::xc_misc::*;

fn main() {

    println!("size of XenSysctlPhysinfo: {}", std::mem::size_of::<XenSysctlPhysinfo>());
    println!("size of PrivCmdHypercall: {}", std::mem::size_of::<PrivCmdHypercall>());
    println!("size of XenSysctl: {}", std::mem::size_of::<XenSysctl>());

    match get_physinfo() {
        Ok(physinfo) => println!("physinfo: {:?}", physinfo),
        Err(err) => println!("err: {}", err),
    }
}