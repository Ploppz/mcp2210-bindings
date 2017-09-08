//! `Bindgen` bindings to the MCP2210-Library.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


#[derive(Copy,Clone)]
pub enum PinDesignation {
    GPIO,
    ChipSelects,
    DedicatedFunction,
}

pub mod consts {
    /// Pin Designation
    pub mod PinDes {
        pub const GPIO: u32 = 0;
        pub const CHIP_SELECTS: u32 = 1;
        pub const DEDICATED_FN: u32 = 2;
    }
    /// GPIO direction
    pub mod GpioDir {
        pub const OUTPUT: u32 = 0;
        pub const INPUT: u32 = 1;
    }
    /// GPIO Output
    pub mod GpioOutput {
        pub const LOW: u32 = 0;
        pub const HIGH: u32 = 1;
    }

    ///
    pub mod EngineStatus {
        /// SPI transfer finished, no data to send
        pub const FINISHED_NO_SEND: u32 = 0x10;
        /// SPI transfer started, no data to receive
        pub const STARTED_NO_RECV: u32 = 0x20;
        /// SPI data accepted, command completed successfully
        pub const ACCEPTED_DONE: u32 = 0x30;
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
