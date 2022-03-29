use crate::device;
use crate::device::uart::Uart;
use crate::device::Device;
use crate::device::RegistersWrapper;
use tock_registers::interfaces::ReadWriteable;
use tock_registers::interfaces::Readable;
use tock_registers::interfaces::Writeable;
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
use tock_registers::{register_bitfields, register_structs};

register_structs! {
    #[allow(non_snake_case)]
    RegisterBlock {
        (0x00 => DR: ReadWrite<u32, DR::Register>),
        (0x04 => _reserved1),
        (0x18 => FR: ReadOnly<u32, FR::Register>),
        (0x1c => _reserved2),
        (0x24 => IBRD: WriteOnly<u32, IBRD::Register>),
        (0x28 => FBRD: WriteOnly<u32, FBRD::Register>),
        (0x2c => LCR_H: ReadWrite<u32, LCR_H::Register>),
        (0x30 => CR: ReadWrite<u32, CR::Register>),
        (0x3C => _reserved3),
        (0x48 => @END),
    }
}

register_bitfields! [u32,
    DR [
        DATA OFFSET(0) NUMBITS(8) [],
        FE OFFSET(8) NUMBITS(1) [],
        PE OFFSET(9) NUMBITS(1) [],
        BE OFFSET(10) NUMBITS(1) [],
        OE OFFSET(11) NUMBITS(1) [],
    ],
    FR [
        CTS 0,
        DSR 1,
        DCD 2,
        BUSY 3,
        RXFE 4,
        TXFF 5,
        RXFF 6,
        TXFE 7,
        RI 8,
    ],
    IBRD [
        BAUD_DIVINT OFFSET(0) NUMBITS(16) []
    ],
    FBRD [
        BAUD_DIVFRAC OFFSET(0) NUMBITS(6) []
    ],
    LCR_H [
        #[allow(clippy::enum_variant_names)]
        WLEN OFFSET(5) NUMBITS(2) [
            FiveBit = 0b00,
            SixBit = 0b01,
            SevenBit = 0b10,
            EightBit = 0b11
        ],

        FEN  OFFSET(4) NUMBITS(1) []
    ],

    CR [
        UARTEN 0,
        TXE 8,
        RXE 9,
    ],
];

type Registers = RegistersWrapper<RegisterBlock>;

/// Result type for PL011 driver
pub type Result<T> = core::result::Result<T, &'static str>;

/// represent a PL011 device
pub struct Pl011 {
    name: &'static str,
    reg: Registers,
    freq: u32,
    baudrate: u32,
}

impl Pl011 {
    /// create new PL011 instace
    pub const fn new(name: &'static str, addr: usize, freq: u32, baudrate: u32) -> Self {
        Self {
            name,
            reg: Registers::new(addr),
            freq,
            baudrate,
        }
    }

    /// change baudrate
    pub fn set_baudrate(&self, baudrate: u32) -> Result<()> {
        // baud rate divisor = BAUDDIV = (FUARTCLK/(16x BaudRate))
        let ibrd = self.freq / (16 * baudrate);
        let fbrd = 64 * (self.freq % (16 * baudrate)) / (16 * baudrate);

        self.reg.IBRD.write(IBRD::BAUD_DIVINT.val(ibrd));
        self.reg.FBRD.write(FBRD::BAUD_DIVFRAC.val(fbrd));

        Ok(())
    }
}

impl Uart for Pl011 {
    type Error = &'static str;

    fn write(&self, data: &[u8]) -> Result<()> {
        for &c in data {
            // wait for room
            while self.reg.FR.is_set(FR::TXFF) {}

            self.reg.DR.write(DR::DATA.val(c as u32));
        }
        Ok(())
    }

    fn read(&self, data: &mut [u8]) -> Result<()> {
        for c in data {
            while self.reg.FR.is_set(FR::RXFE) {}

            *c = self.reg.DR.get() as u8;
        }

        Ok(())
    }
}

impl Device for Pl011 {
    fn class(&self) -> device::Class {
        device::Class::Uart
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn init(&self) {
        self.set_baudrate(self.baudrate).unwrap();

        // enable FIFO
        self.reg
            .LCR_H
            .modify(LCR_H::WLEN::EightBit + LCR_H::FEN::SET);

        // enable UART
        self.reg
            .CR
            .modify(CR::UARTEN::SET + CR::TXE::SET + CR::RXE::SET);
    }
}
