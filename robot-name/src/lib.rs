use std::fmt::{Display, Formatter};

struct RobotName {
    l: u32,
    d: u32,
}

impl Display for RobotName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let first = 'A' as u8 + (self.l / Self::LETTER_COUNT) as u8;
        let second = 'A' as u8 + (self.l % Self::LETTER_COUNT) as u8;
        let third = format!("{:03}", self.d);
        write!(f, "{}{}{:03}", first as char, second as char, third)
    }
}

impl RobotName {
    const LETTER_COUNT: u32 = 26;
    const LETTER_LIMIT: u32 = Self::LETTER_COUNT * Self::LETTER_COUNT;
    const DIGIT_LIMIT: u32 = 1000;
    fn generate_name(&mut self) -> String {
        let rem_d = (self.d + 1) / Self::DIGIT_LIMIT;
        self.d = (self.d + 1) % Self::DIGIT_LIMIT;

        if rem_d > 0 {
            let rem_l = (self.l + rem_d) / Self::LETTER_LIMIT;
            self.l = (self.l + rem_d) % Self::LETTER_LIMIT;
            if rem_l > 0 {
                self.d = 0;
            }
        }
        self.to_string()
    }
}

static mut ROBOT_NAME: RobotName = RobotName { l: 0, d: 0 };

pub struct Robot(String);

impl Robot {
    pub fn new() -> Self {
        Self(unsafe { ROBOT_NAME.generate_name() })
    }

    pub fn name(&self) -> &str {
        self.0.as_str()
    }

    pub fn reset_name(&mut self) {
        self.0 = unsafe { ROBOT_NAME.generate_name() }
    }
}
