use core::ptr::{read_volatile, write_volatile};

// Memory Map and Registers (ADJUST THESE BASED ON YOUR ESP32 AND FLASH CHIP)
const SPI_FLASH_BASE: u32 = 0x3ff42000; // Correct base address for SPI1
const SPI_FLASH_CMD_REG: u32 = SPI_FLASH_BASE + 0x00;
const SPI_FLASH_ADDR_REG: u32 = SPI_FLASH_BASE + 0x04;
const SPI_FLASH_DATA_REG: u32 = SPI_FLASH_BASE + 0x80; // Adjusted offset for data register
const SPI_FLASH_STATUS_REG: u32 = SPI_FLASH_BASE + 0x10; // Example Status Register

// Flash Commands (ADJUST THESE BASED ON YOUR FLASH CHIP'S DATASHEET)
const FLASH_CMD_READ_DATA: u8 = 0x03;
const FLASH_CMD_WRITE_ENABLE: u8 = 0x06;
const FLASH_CMD_PAGE_PROGRAM: u8 = 0x02;
const FLASH_CMD_ERASE_SECTOR: u8 = 0x20;
const FLASH_CMD_READ_STATUS_REG_1: u8 = 0x05;

// Status Register Bits (ADJUST THESE BASED ON YOUR FLASH CHIP'S DATASHEET)
const STATUS_WIP_BIT: u8 = 0x01; // Write In Progress

pub struct FlashStorage {
    address: u32,
}

impl FlashStorage {
    pub fn new() -> Self {
        Self { address: 0x3F0000 } // Example start address
    }

    fn wait_for_ready(&self) -> Result<(), &'static str> {
        unsafe {
            loop {
                write_volatile(SPI_FLASH_CMD_REG as *mut u8, FLASH_CMD_READ_STATUS_REG_1);
                let status = read_volatile(SPI_FLASH_DATA_REG as *const u8);
                if (status & STATUS_WIP_BIT) == 0 {
                    break; // Ready
                }
            }
        }
        Ok(())
    }

    pub fn erase_sector(&mut self, sector_address: u32) -> Result<(), &'static str> {
        self.wait_for_ready()?;
        unsafe {
            write_volatile(SPI_FLASH_CMD_REG as *mut u8, FLASH_CMD_WRITE_ENABLE);
            write_volatile(SPI_FLASH_CMD_REG as *mut u8, FLASH_CMD_ERASE_SECTOR);
            write_volatile(SPI_FLASH_ADDR_REG as *mut u32, sector_address);
        }
        self.wait_for_ready()
    }

    pub fn read_bytes(&self, address: u32, buffer: &mut [u8]) -> Result<(), &'static str> {
        self.wait_for_ready()?;
        unsafe {
            write_volatile(SPI_FLASH_CMD_REG as *mut u8, FLASH_CMD_READ_DATA);
            write_volatile(SPI_FLASH_ADDR_REG as *mut u32, address);
            for byte in buffer.iter_mut() {
                *byte = read_volatile(SPI_FLASH_DATA_REG as *const u8);
            }
        }
        Ok(())
    }

    pub fn write_bytes(&mut self, address: u32, data: &[u8]) -> Result<(), &'static str> {
        self.wait_for_ready()?;
        for chunk in data.chunks(256) {
            unsafe {
                // Enable write operations
                write_volatile(SPI_FLASH_CMD_REG as *mut u8, FLASH_CMD_WRITE_ENABLE);
                write_volatile(SPI_FLASH_CMD_REG as *mut u8, FLASH_CMD_PAGE_PROGRAM);
                write_volatile(SPI_FLASH_ADDR_REG as *mut u32, address);
                for &byte in chunk {
                    write_volatile(SPI_FLASH_DATA_REG as *mut u8, byte);
                }
            }
            self.wait_for_ready()?;
        }
        Ok(())
    }
}
