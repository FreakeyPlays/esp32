const OFFSET: u32 = 0x300000;

#[derive(Debug)]
pub struct FlashStorage {
    base_address: u32,
}

impl FlashStorage {
    pub fn new(base_address: u32) -> Self {
        FlashStorage { base_address }
    }

    /// Schreibt Daten in den Speicher.
    pub fn write_to_memory(&self, data: &[u8]) {
        let loc = self.base_address + OFFSET;

        unsafe {
            let ptr = loc as *mut u8;
            for (i, byte) in data.iter().enumerate() {
                core::ptr::write_volatile(ptr.add(i), *byte);
            }
        }
    }

    /// Liest Daten aus dem Speicher.
    pub fn read_from_memory(&self, buffer: &mut [u8]) {
        let loc = self.base_address + OFFSET;

        unsafe {
            let ptr = loc as *const u8;
            for (i, byte) in buffer.iter_mut().enumerate() {
                *byte = core::ptr::read_volatile(ptr.add(i));
            }
        }
    }
}
