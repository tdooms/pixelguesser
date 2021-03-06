// Removed i, I, o, O -> 48 chars
// !! MUST be sorted by ascii values
static CHARS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZabcdefghjklmnpqrstuvwxyz";

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Code {
    pub session_id: u32,
    pub quiz_id: u32,
}

impl std::str::FromStr for Code {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut code = 0u64;

        for char in s.chars() {
            let index = CHARS.iter().position(|c| char::from(*c) == char).ok_or(())?;
            code = code * (CHARS.len() as u64) + (index as u64);
        }

        // It is important the quiz id is the last part of the code in binary (most significant bits)
        // As quiz id's are usually small, this reduces the length of the ascii code.
        Ok(Code { session_id: (code & 0x00000000FFFFFFFF) as u32, quiz_id: (code >> 32) as u32 })
    }
}

impl ToString for Code {
    fn to_string(&self) -> String {
        let mut string = String::new();
        let mut code = self.session_id as u64 + ((self.quiz_id as u64) << 32);

        while code != 0 {
            let rem = code % CHARS.len() as u64;
            string.insert(0, char::from(CHARS[rem as usize]));
            code /= CHARS.len() as u64;
        }
        string
    }
}
