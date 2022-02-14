use std::str::FromStr;

// Removed i, I, o, O -> 48 chars
// !! MUST be sorted by ascii values
static CHARS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZabcdefghjklmnpqrstuvwxyz";

pub struct Code {
    pub session_id: u64,
    pub quiz_id: u64,
}

impl FromStr for Code {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut code = 0u64;

        for char in s.chars() {
            let index = CHARS.iter().position(|c| char::from(*c) == char).ok_or(())?;
            code = code * (CHARS.len() as u64) + (index as u64);
        }

        Ok(Code { session_id: code & 0x00000000FFFFFFFF, quiz_id: code >> 32 })
    }
}

impl ToString for Code {
    fn to_string(&self) -> String {
        let mut string = String::new();
        let mut code = self.session_id + (self.quiz_id << 32);

        while code != 0 {
            let rem = code % CHARS.len() as u64;
            string.insert(0, char::from(CHARS[rem as usize]));
            code /= CHARS.len() as u64;
        }
        string
    }
}
