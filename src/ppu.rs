// A real implementation would need access to the cartridge (CHR ROM) and a bus.
// For now, we'll keep it self-contained.

pub struct PPU {
    // PPU Memory
    vram: [u8; 2048], // 2KB of Video RAM for nametables
    oam_data: [u8; 256], // Object Attribute Memory for 64 sprites
    palette_ram: [u8; 32], // Color palette memory

    // Internal Registers for scrolling and VRAM access
    v: u16, // Current VRAM address (15 bits)
    t: u16, // Temporary VRAM address (15 bits); the "tram" address
    x: u8,  // Fine X scroll (3 bits)
    w: bool, // Write toggle (for PPUADDR/PPUSCROLL)

    // CPU-Visible Registers
    // We model the state they affect rather than storing the raw values for most.
    ppuctrl: u8,
    ppumask: u8,
    ppustatus: u8,
    oam_addr: u8,

    // Rendering State
    scanline: i16,
    cycle: u16,

    // Data buffer for PPUDATA reads
    data_buffer: u8,
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            vram: [0; 2048],
            oam_data: [0; 256],
            palette_ram: [0; 32],
            v: 0,
            t: 0,
            x: 0,
            w: false,
            ppuctrl: 0,
            ppumask: 0,
            ppustatus: 0,
            oam_addr: 0,
            scanline: -1, // Start on the pre-render scanline
            cycle: 0,
            data_buffer: 0,
        }
    }

    /// Handles CPU reads from PPU registers ($2000-$2007)
    pub fn cpu_read(&mut self, addr: u16) -> u8 {
        match addr {
            0x2002 => { // PPUSTATUS
                // Reading status register clears the VBlank flag and the address latch
                let status = self.ppustatus;
                self.ppustatus &= 0b0111_1111; // Clear VBlank flag
                self.w = false; // Reset address latch
                status
            }
            0x2004 => { // OAMDATA
                self.oam_data[self.oam_addr as usize]
            }
            0x2007 => { // PPUDATA
                let mut data = self.vram[(self.v & 0x3FFF) as usize]; // Placeholder, needs mirroring logic
                if self.v < 0x3F00 {
                    // Reads from VRAM are buffered, so the first read is invalid
                    let buffered_data = self.data_buffer;
                    self.data_buffer = data;
                    data = buffered_data;
                } else {
                    // Palette RAM reads are not buffered
                    self.data_buffer = self.vram[(self.v - 0x1000) as usize]; // Mirror down
                }
                // Increment VRAM address after read/write, controlled by PPUCTRL
                self.v += if (self.ppuctrl & 0b100) == 0 { 1 } else { 32 };
                data
            }
            _ => 0, // Other registers are write-only or have no readable value
        }
    }

    /// Handles CPU writes to PPU registers ($2000-$2007)
    pub fn cpu_write(&mut self, addr: u16, data: u8) {
        match addr {
            0x2000 => { // PPUCTRL
                self.ppuctrl = data;
                // Update temporary VRAM address with nametable bits from control register
                self.t = (self.t & 0xF3FF) | ((data as u16 & 0x03) << 10);
            }
            0x2001 => { // PPUMASK
                self.ppumask = data;
            }
            0x2003 => { // OAMADDR
                self.oam_addr = data;
            }
            0x2004 => { // OAMDATA
                self.oam_data[self.oam_addr as usize] = data;
                self.oam_addr = self.oam_addr.wrapping_add(1);
            }
            0x2005 => { // PPUSCROLL
                if !self.w { // First write
                    self.t = (self.t & 0xFFE0) | (data as u16 >> 3);
                    self.x = data & 0x07;
                    self.w = true;
                } else { // Second write
                    self.t = (self.t & 0x8C1F) | ((data as u16 & 0xF8) << 2) | ((data as u16 & 0x07) << 12);
                    self.w = false;
                }
            }
            0x2006 => { // PPUADDR
                if !self.w { // First write
                    self.t = (self.t & 0x00FF) | ((data as u16 & 0x3F) << 8);
                    self.w = true;
                } else { // Second write
                    self.t = (self.t & 0xFF00) | (data as u16);
                    self.v = self.t; // On second write, copy temp address to main address
                    self.w = false;
                }
            }
            0x2007 => { // PPUDATA
                self.vram[(self.v & 0x3FFF) as usize] = data; // Placeholder, needs mirroring
                // Increment VRAM address after read/write, controlled by PPUCTRL
                self.v += if (self.ppuctrl & 0b100) == 0 { 1 } else { 32 };
            }
            _ => {}
        }
    }

    /// Executes one PPU cycle.
    /// This is where all the rendering magic will happen.
    pub fn step(&mut self) {
        // TODO: Implement PPU rendering cycle by cycle.
        // This involves:
        // - Incrementing cycle and scanline counters.
        // - Handling different parts of a scanline (visible, hblank, vblank).
        // - Fetching nametable data, pattern table data, and attribute data.
        // - Evaluating sprites for the next scanline.
        // - Outputting a pixel to a frame buffer.
        // - Setting PPUSTATUS flags (VBlank, sprite overflow, etc.).
        // - Triggering NMI on VBlank if enabled in PPUCTRL.
    }
}
