pub struct BPIALLAccessor;
pub const BPIALL: BPIALLAccessor = BPIALLAccessor;
impl tock_registers::interfaces::Writeable for BPIALLAccessor {
    type T = u32;
    type R = ();
    coproc_write_raw!(p15, 0, value, c7, c5, 6);
}

pub struct BPIALLISAccessor;
pub const BPIALLIS: BPIALLISAccessor = BPIALLISAccessor;
impl tock_registers::interfaces::Writeable for BPIALLISAccessor {
    type T = u32;
    type R = ();
    coproc_write_raw!(p15, 0, value, c7, c1, 6);
}

pub struct BPIMVAAccessor;
pub const BPIMVA: BPIMVAAccessor = BPIMVAAccessor;
impl tock_registers::interfaces::Writeable for BPIMVAAccessor {
    type T = u32;
    type R = ();
    coproc_write_raw!(p15, 0, value, c7, c5, 7);
}

pub struct DCCIMVACAccessor;
pub const DCCIMVAC: DCCIMVACAccessor = DCCIMVACAccessor;
impl tock_registers::interfaces::Writeable for DCCIMVACAccessor {
    type T = u32;
    type R = ();
    coproc_write_raw!(p15, 0, value, c7, c14, 1);
}

pub struct DCCISWAccessor;
pub const DCCISW: DCCISWAccessor = DCCISWAccessor;
impl tock_registers::interfaces::Writeable for DCCISWAccessor {
    type T = u32;
    type R = ();
    coproc_write_raw!(p15, 0, value, c7, c14, 2);
}

pub struct DCCMVACAccessor;
pub const DCCMVAC: DCCMVACAccessor = DCCMVACAccessor;
impl tock_registers::interfaces::Writeable for DCCMVACAccessor {
    type T = u32;
    type R = ();
    coproc_write_raw!(p15, 0, value, c7, c10, 1);
}

pub struct DCCMVAUAccessor;
pub const DCCMVAU: DCCMVAUAccessor = DCCMVAUAccessor;
impl tock_registers::interfaces::Writeable for DCCMVAUAccessor {
    type T = u32;
    type R = ();
    coproc_write_raw!(p15, 0, value, c7, c11, 1);
}

pub struct DCCSWAccessor;
pub const DCCSW: DCCSWAccessor = DCCSWAccessor;
impl tock_registers::interfaces::Writeable for DCCSWAccessor {
    type T = u32;
    type R = ();
    coproc_write_raw!(p15, 0, value, c7, c10, 2);
}

pub struct DCIMVACAccessor;
pub const DCIMVAC: DCIMVACAccessor = DCIMVACAccessor;
impl tock_registers::interfaces::Writeable for DCIMVACAccessor {
    type T = u32;
    type R = ();
    coproc_write_raw!(p15, 0, value, c7, c6, 1);
}

pub struct DCISWAccessor;
pub const DCISW: DCISWAccessor = DCISWAccessor;
impl tock_registers::interfaces::Writeable for DCISWAccessor {
    type T = u32;
    type R = ();
    coproc_write_raw!(p15, 0, value, c7, c6, 2);
}

pub struct ICIALLUAccessor;
pub const ICIALLU: ICIALLUAccessor = ICIALLUAccessor;
impl tock_registers::interfaces::Writeable for ICIALLUAccessor {
    type T = u32;
    type R = ();
    coproc_write_raw!(p15, 0, value, c7, c5, 0);
}

pub struct ICIALLUISAccessor;
pub const ICIALLUIS: ICIALLUISAccessor = ICIALLUISAccessor;
impl tock_registers::interfaces::Writeable for ICIALLUISAccessor {
    type T = u32;
    type R = ();
    coproc_write_raw!(p15, 0, value, c7, c1, 0);
}

pub struct ICIMVAUAccessor;
pub const ICIMVAU: ICIMVAUAccessor = ICIMVAUAccessor;
impl tock_registers::interfaces::Writeable for ICIMVAUAccessor {
    type T = u32;
    type R = ();
    coproc_write_raw!(p15, 0, value, c7, c5, 1);
}
