/*
generate passcode from callsign for APRS-IS
*/
pub fn generate(callsign: &str) -> u32 {
    let passcode = callsign.split("-").nth(0).unwrap();
    // initialize hash
    let mut hash = 0x73e2;
    let mut chars = passcode.chars();
    let mut i = 0;
    let count = chars.clone().count();

    while i < count {
        hash ^= ord(chars.nth(0).unwrap()) << 8;
        hash ^= ord(chars.nth(0).unwrap());
        i += 2;
    }

    // mask off the high bit so number is always positive
    hash & 0x7fff
}

/*
get ascii value for a character
*/
fn ord(c: char) -> u32 {
    c as u32
}
