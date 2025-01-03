/// let mut store: store::Store = store::Store::new();
/// let cp1 = Chip::new([1, 2, 3, 4], "chris");
/// if let Err(e) = store.add_chip(cp1) {
///     info!("Failed to load chip: {:?}", e);
/// }
/// if let Err(e) = store.remove_chip_by_id([1, 2, 3, 4]) {
///     info!("Failed to delete chip: {:?}", e);
/// }
/// for i in 0..10 {
///     let log1 = Log::new([1, 2, 3, 4], "chris", i, 0);
///
///     if let Err(e) = store.add_log(log1) {
///         info!("Failed to load chip: {:?}", e);
///     }
/// }
///
/// let log1 = Log::new([99, 99, 99, 99], "test", 1, 2);
///
/// if let Err(e) = store.add_log(log1) {
///     info!("Failed to load chip: {:?}", e);
/// }
/// info!("Store: {:?}", store);
///
use core::mem::size_of;
use core::slice;

use log::info;

use crate::lib::flash::FlashStorage;

const CHIP_COUNT: usize = 10;
const LOG_COUNT: usize = 10;

// We use the Arrays instead of the whole Object in case of Padding or other values
const CHIP_SIZE: usize = size_of::<Chip>();
const LOG_SIZE: usize = size_of::<Log>();
const TOTAL_STORE_SIZE: usize = CHIP_SIZE * CHIP_COUNT + LOG_SIZE * LOG_COUNT;

pub type Name = [u8; 5];
pub type Id = [u8; 4];

/// Repräsentiert einen Chip
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Chip {
    id: Id,
    name: Name,
}

impl Chip {
    pub fn new(id: Id, name: &str) -> Self {
        let mut name_array = [0u8; 5];
        let name_bytes = name.as_bytes();
        let len = name_bytes.len().min(5);
        name_array[..len].copy_from_slice(&name_bytes[..len]);
        Self {
            id,
            name: name_array,
        }
    }

    const EMPTY: Chip = Chip {
        id: [0, 0, 0, 0],
        name: [0, 0, 0, 0, 0],
    };
}

/// Repräsentiert ein Log
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Log {
    user_id: Id,
    name: Name,
    gate_id: u32,
    state: u8,
}

impl Log {
    pub fn new(user_id: Id, name: &str, gate_id: u32, state: u8) -> Self {
        let mut name_array = [0u8; 5];
        let name_bytes = name.as_bytes();
        let len = name_bytes.len().min(5);
        name_array[..len].copy_from_slice(&name_bytes[..len]);
        Self {
            user_id,
            name: name_array,
            gate_id,
            state,
        }
    }

    const EMPTY: Log = Log {
        user_id: [0, 0, 0, 0],
        name: [0, 0, 0, 0, 0],
        gate_id: 0,
        state: 0,
    };
}

/// Speichert Chips und Logs
#[derive(Debug)]
pub struct Store {
    flash_storage: FlashStorage,
    chips: [Chip; CHIP_COUNT],
    logs: [Log; LOG_COUNT],
}

impl Store {
    pub fn new(base_address: u32) -> Self {
        Self {
            flash_storage: FlashStorage::new(base_address),
            chips: [Chip::default(); CHIP_COUNT],
            logs: [Log::default(); LOG_COUNT],
        }
    }

    pub fn save(&self) -> Result<(), &'static str> {
        let mut buffer = [0u8; TOTAL_STORE_SIZE];
        self.to_bytes(&mut buffer)?;
        info!("Write buffer: {:?}", buffer);
        self.flash_storage.write_to_memory(&buffer);
        Ok(())
    }

    pub fn load(&mut self) -> Result<(), &'static str> {
        let mut buffer = [0u8; TOTAL_STORE_SIZE];
        self.flash_storage.read_from_memory(&mut buffer);
        info!("Read buffer: {:?}", buffer);
        *self = Self::new_from_bytes(&buffer);
        Ok(())
    }

    pub fn new_from_bytes(buffer: &[u8; TOTAL_STORE_SIZE]) -> Self {
        let mut chips: [Chip; CHIP_COUNT] = [Chip::default(); CHIP_COUNT];
        let mut logs: [Log; LOG_COUNT] = [Log::default(); LOG_COUNT];

        let mut offset = 0;

        for chip in chips.iter_mut() {
            let chip_bytes = &buffer[offset..offset + CHIP_SIZE];
            *chip = unsafe { core::ptr::read_unaligned(chip_bytes.as_ptr() as *const Chip) };
            offset += CHIP_SIZE;
        }

        for log in logs.iter_mut() {
            let log_bytes = &buffer[offset..offset + LOG_SIZE];
            *log = unsafe { core::ptr::read_unaligned(log_bytes.as_ptr() as *const Log) };
            offset += LOG_SIZE;
        }

        Self {
            flash_storage: FlashStorage::new(0), // Wird beim tatsächlichen Gebrauch überschrieben
            chips,
            logs,
        }
    }

    pub fn to_bytes(&self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        let total_size = CHIP_SIZE * CHIP_COUNT + LOG_SIZE * LOG_COUNT;

        if buffer.len() < total_size {
            return Err("Buffer too small");
        }

        let mut offset = 0;

        for chip in &self.chips {
            let chip_bytes =
                unsafe { slice::from_raw_parts(chip as *const Chip as *const u8, CHIP_SIZE) };
            buffer[offset..offset + CHIP_SIZE].copy_from_slice(chip_bytes);
            offset += CHIP_SIZE;
        }

        for log in &self.logs {
            let log_bytes =
                unsafe { slice::from_raw_parts(log as *const Log as *const u8, LOG_SIZE) };
            buffer[offset..offset + LOG_SIZE].copy_from_slice(log_bytes);
            offset += LOG_SIZE;
        }

        Ok(total_size)
    }

    pub fn add_chip(&mut self, chip: Chip) -> Result<(), &'static str> {
        for c in self.chips.iter_mut() {
            if *c == Chip::EMPTY {
                *c = chip;
                return Ok(());
            }
        }
        Err("Chip storage is full")
    }

    pub fn remove_chip_by_id(&mut self, id: Id) -> Result<(), &'static str> {
        for chip in self.chips.iter_mut() {
            if chip.id == id {
                *chip = Chip::EMPTY;
                return Ok(());
            }
        }
        Err("Chip with given ID not found")
    }

    pub fn add_log(&mut self, log: Log) -> Result<(), &'static str> {
        for l in self.logs.iter_mut() {
            if *l == Log::EMPTY {
                *l = log;
                return Ok(());
            }
        }
        for i in 0..LOG_COUNT - 1 {
            self.logs[i] = self.logs[i + 1];
        }
        self.logs[LOG_COUNT - 1] = log;
        Ok(())
    }
}
