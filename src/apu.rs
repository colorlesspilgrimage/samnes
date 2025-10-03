pub struct Pulse {
    // Registers: $4000, $4001, $4002, $4003
    duty: u8,
    length_counter_halt: bool, // Also envelope loop
    constant_volume: bool,     // Also envelope flag
    volume: u8,                // Also envelope period

    sweep_enabled: bool,
    sweep_period: u8,
    sweep_negate: bool,
    sweep_shift: u8,

    timer: u16,
    length_counter: u8,
}

pub struct Triangle {
    // Registers: $4008, $400A, $400B
    control_flag: bool, // Also linear counter halt
    linear_counter_load: u8,

    timer: u16,
    length_counter: u8,
}

pub struct Noise {
    // Registers: $400C, $400E, $400F
    length_counter_halt: bool, // Also envelope loop
    constant_volume: bool,
    volume: u8, // Also envelope period

    mode: bool, // Loop noise
    period: u8,

    length_counter: u8,
}

pub struct DMC {
    // Registers: $4010, $4011, $4012, $4013
    irq_enabled: bool,
    loop_flag: bool,
    frequency: u8,

    load_counter: u8,
    sample_address: u8,
    sample_length: u8,
}

pub struct APU {
    pulse1: Pulse,
    pulse2: Pulse,
    triangle: Triangle,
    noise: Noise,
    dmc: DMC,

    // Global control
    status: u8, // $4015
    frame_counter: u8, // $4017
}

impl APU {
    pub fn new() -> Self {
        // Initialize all channels to a silent, powered-off state
        APU {
            pulse1: Pulse { duty: 0, length_counter_halt: false, constant_volume: false, volume: 0, sweep_enabled: false, sweep_period: 0, sweep_negate: false, sweep_shift: 0, timer: 0, length_counter: 0 },
            pulse2: Pulse { duty: 0, length_counter_halt: false, constant_volume: false, volume: 0, sweep_enabled: false, sweep_period: 0, sweep_negate: false, sweep_shift: 0, timer: 0, length_counter: 0 },
            triangle: Triangle { control_flag: false, linear_counter_load: 0, timer: 0, length_counter: 0 },
            noise: Noise { length_counter_halt: false, constant_volume: false, volume: 0, mode: false, period: 0, length_counter: 0 },
            dmc: DMC { irq_enabled: false, loop_flag: false, frequency: 0, load_counter: 0, sample_address: 0, sample_length: 0 },
            status: 0,
            frame_counter: 0,
        }
    }

    /// Handles CPU writes to APU registers ($4000-$4017)
    pub fn cpu_write(&mut self, addr: u16, data: u8) {
        match addr {
            // Pulse 1: $4000-$4003
            0x4000 => { /* Control */ }
            0x4001 => { /* Sweep */ }
            0x4002 => { /* Timer low */ }
            0x4003 => { /* Timer high / Length counter */ }

            // Pulse 2: $4004-$4007
            0x4004 => { /* Control */ }
            0x4005 => { /* Sweep */ }
            0x4006 => { /* Timer low */ }
            0x4007 => { /* Timer high / Length counter */ }

            // Triangle: $4008-$400B
            0x4008 => { /* Control */ }
            0x4009 => { /* Unused */ }
            0x400A => { /* Timer low */ }
            0x400B => { /* Timer high / Length counter */ }

            // Noise: $400C-$400F
            0x400C => { /* Control */ }
            0x400D => { /* Unused */ }
            0x400E => { /* Period */ }
            0x400F => { /* Length counter */ }

            // DMC: $4010-$4013
            0x4010 => { /* Control */ }
            0x4011 => { /* Load counter */ }
            0x4012 => { /* Sample address */ }
            0x4013 => { /* Sample length */ }

            // Status Register
            0x4015 => { self.status = data; }

            // Frame Counter
            0x4017 => { self.frame_counter = data; }

            _ => {}
        }
    }

    /// Ticks the APU state forward.
    pub fn tick(&mut self) {
        // TODO: Implement APU clocking.
        // The APU is complex. It has its own timing based on the CPU clock.
        // A frame counter clocks the envelopes and length counters at 240Hz.
        // Timers for each channel determine the frequency of the sound wave they produce.
    }
}
