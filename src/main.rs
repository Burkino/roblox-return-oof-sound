extern crate winreg;
use std::{io, fs};
use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

static SOUND: &'static [u8] = include_bytes!("./ouch.ogg");
fn main() -> io::Result<()> {
    let hkci = RegKey::predef(HKEY_CURRENT_USER);
    let rbx_key = hkci.open_subkey("SOFTWARE\\ROBLOX Corporation\\Environments\\roblox-player")?;
    let path: String = rbx_key.get_value("")?;
    let file: String = format!("{}\\..\\content\\sounds\\ouch.ogg", &path);
    fs::write(&file, SOUND)?;
    Ok(())
}
