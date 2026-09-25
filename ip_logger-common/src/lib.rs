#![no_std]


#[repr(C)]
pub struct IpPair {
    pub src: u32,
    pub dst: u32,
}