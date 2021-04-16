use std::ffi::CString;

pub(crate) trait FlString {
    fn safe_new(s: &str) -> CString;
}

impl FlString for CString {
    fn safe_new(s: &str) -> CString {
        match CString::new(s) {
            Ok(v) => v,
            Err(r) => {
                let i = r.nul_position();
                CString::new(&r.into_vec()[0..i]).unwrap()
            }
        }
    }
}

/// Convenience function to convert rgb to hex.
/// Example:
/// ```rust,no_run
/// use fltk::utils::rgb2hex;
/// let ret = rgb2hex(0, 255, 0); println!("0x{:06x}", ret);
/// ```
pub fn rgb2hex(r: u8, g: u8, b: u8) -> u32 {
    // Shouldn't fail
    u32::from_str_radix(&format!("{:02x}{:02x}{:02x}", r, g, b), 16).unwrap()
}

/// Convenience function to convert rgba to hex.
/// Example:
/// ```rust,no_run
/// use fltk::utils::rgba2hex;
/// let ret = rgba2hex(0, 255, 0, 255); println!("0x{:08x}", ret);
/// ```
pub fn rgba2hex(r: u8, g: u8, b: u8, a: u8) -> u32 {
    // Shouldn't fail
    u32::from_str_radix(&format!("{:02x}{:02x}{:02x}{:02x}", r, g, b, a), 16).unwrap()
}

/// Convenience function to convert hex to rgb.
/// Example:
/// ```rust,no_run
/// use fltk::utils::hex2rgb;
/// let (r, g, b) = hex2rgb(0x000000);
/// ```
pub fn hex2rgb(val: u32) -> (u8, u8, u8) {
    let hex = format!("{:06x}", val);
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    (r, g, b)
}

/// Convenience function to convert hex to rgba.
/// Example:
/// ```rust,no_run
/// use fltk::utils::hex2rgba;
/// let (r, g, b, a) = hex2rgba(0xff0000ff);
/// ```
pub fn hex2rgba(val: u32) -> (u8, u8, u8, u8) {
    let r = ((val >> 24) & 0xff) as u8;
    let g = ((val >> 16) & 0xff) as u8;
    let b = ((val >> 8) & 0xff) as u8;
    let a = (val & 0xff) as u8;
    (r, g, b, a)
}
