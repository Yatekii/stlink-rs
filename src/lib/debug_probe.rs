use dbg_probe::protocol::WireProtocol;

pub trait DebugProbe {
    type Error;

    fn open(&mut self) -> Result<(), Self::Error>;

    fn close(&mut self) -> Result<(), Self::Error>;

    fn get_version(&mut self) -> Result<(u8, u8), Self::Error>;

    /// Enters debug mode
    fn attach(&mut self, protocol: WireProtocol) -> Result<(), Self::Error>;

    /// Leave debug mode
    fn detach(&mut self) -> Result<(), Self::Error>;

    fn target_reset(&mut self) -> Result<(), Self::Error>;

    /// Reads the DAP register on the specified port and address.
    fn read_dap_register(&mut self, port: u16, addr: u32) -> Result<u32, Self::Error>;

    /// Writes a value to the DAP register on the specified port and address.
    fn write_dap_register(&mut self, port: u16, addr: u16, value: u32) -> Result<(), Self::Error>;
}
