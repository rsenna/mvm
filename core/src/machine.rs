use crate::hart::{Hart, SimpleHart};

// Init memory as 128MB
pub const DRAM_SIZE: usize = 1024 * 1024 * 128;

pub type Byte = u8;

// TODO make dram available to the Hart
// TODO implement a *true* shareable memory between different processes
pub struct Machine {
    pub hart: Box<dyn Hart>,
}

impl Machine {
    pub fn new() -> Self {
        let ram = vec![0; DRAM_SIZE];

        Self {
            hart: Box::new(SimpleHart::new(ram)),
        }
    }
}
