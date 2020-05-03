/// Representation of a color model stored as RGB channels.
///
/// This allows for setting a custom color depth.
///
/// # Please note
///
/// The **depth** is the value for **one channel**,
/// e.g. if you have an depth of 4 => one channel has a range of *2^4 = 16* values
/// but since there are 3 channels in RGB, altogether you have *3 * 4 = 12* bit for the color.
///
/// Since every channel is stored as `u32`, depth cannot be greater than 32.
///
/// If you just mean to use the commonly used 24 bit system be aware that you can simply use `RGB24`.
#[derive(Debug)]
pub struct RGB {
    r: u32,
    g: u32,
    b: u32,

    depth: u8,
}

impl RGB {
    /// Creates a new `RGB`
    ///
    /// # Arguments
    ///
    /// - r, g, b -> values for the channels
    /// - depth -> the bit depth for one channel
    pub fn from_rgb(r: u32, g: u32, b: u32, depth: u8) -> Self {
        RGB { r, g, b, depth }
    }

    /// Returns the value of channel **red**
    pub fn r(&self) -> u32 {
        self.r
    }

    /// Returns the value of channel **green**
    pub fn g(&self) -> u32 {
        self.g
    }

    /// Returns the value of channel **blue**
    pub fn b(&self) -> u32 {
        self.b
    }

    /// Returns the depth in bit for one channel
    pub fn depth(&self) -> u8 {
        self.depth
    }

    /// Sets the value of channel **red**
    pub fn set_r(&mut self, r: u32) {
        self.r = r
    }

    /// Sets the value of channel **green**
    pub fn set_g(&mut self, g: u32) {
        self.g = g
    }

    /// Sets the value of channel **blue**
    pub fn set_b(&mut self, b: u32) {
        self.b = b
    }
}
