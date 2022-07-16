extern crate winreg;
use winreg::RegKey;
use winreg::enums::*;
use std::io;

// Checks completed pictures in Coloring Book for Adults (steam appid 1431240).
// Any picture with at least a value of 50 (clicks) counts as finished and towards the achievements.
pub(crate) fn check_progress() -> io::Result<String> {
    let mut result="Error parsing registry keys.";
    let system = RegKey::predef(HKEY_CURRENT_USER).open_subkey("Software\\DefaultCompany\\Coloring Book for Adults")?;
    for (name, value) in system.enum_values().map(|x| x.unwrap()) {
        if name.contains("counter") {
            let (left, _right) = name.split_once("counter_h").unwrap();
            let i = match value.to_string().parse::<i32>() {
                Ok(i) => i,
                Err(_e) => -1,
            };
            match i {
                0..=49 => println!("{} = {:?} ({})", left.replace("_", " "), i, "✘"),
                _ => println!("{} = {:?} ({})", left.replace("_", " "), i, "✔"),
            }
            result = "Registry keys parsed successfully.";
        }
    }
    println!();
    Ok(result.to_string())
}