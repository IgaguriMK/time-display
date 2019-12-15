use std::fmt;
use std::ops::{Add, AddAssign};

use tiny_fail::Fail;

const PRECISION: u64 = 1_000_000;
const PRECISION_F64: f64 = PRECISION as f64;

/// A type to represents time.
///
/// Precision is micro second.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Time(u64);

impl Time {
    /// Returns time `0.0 s`.
    pub const fn zero() -> Time {
        Time(0)
    }

    pub fn parse(s: &str) -> Result<Time, Fail> {
        let parts: Vec<&str> = s.split(':').collect();

        match parts.len() {
            0 => unreachable!("splitted string should has part."),
            1 => Time::parse_parts("0", parts[0]),
            2 => Time::parse_parts(parts[0], parts[1]),
            _ => Err(Fail::new(format!(
                "Invalid time: \"{}\" has too many parts.",
                s
            ))),
        }
    }

    fn parse_parts(min: &str, sec: &str) -> Result<Time, Fail> {
        let m = min
            .parse::<u64>()
            .map_err(|_| Fail::new(format!("Invalid minutes: \"{}\"", min)))?;
        let s = sec
            .parse::<f64>()
            .map_err(|_| Fail::new(format!("Invalid seconds: \"{}\"", sec)))?;

        let t = PRECISION * 60 * m + ((s * PRECISION_F64) as u64);
        Ok(Time(t))
    }

    pub fn secs_f64(self) -> f64 {
        (self.0 as f64) / PRECISION_F64
    }

    pub fn minute(self) -> u64 {
        self.0 / (PRECISION * 60)
    }

    pub fn second(self) -> f64 {
        ((self.0 % (60 * PRECISION)) as f64) / PRECISION_F64
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let m = self.minute();
        let s = self.second();

        match m {
            0 => {
                let sec_part = format!("{:>5.2}", s);
                if sec_part == "60.00" {
                    write!(f, " 1:00.00")
                } else {
                    write!(f, "  :{}", sec_part)
                }
            }
            m => {
                let sec_part = format!("{:>05.2}", s);
                if sec_part == "60.00" {
                    write!(f, "{:>2}:00.00", m + 1)
                } else {
                    write!(f, "{:>2}:{}", m, sec_part)
                }
            }
        }
    }
}

impl From<f64> for Time {
    fn from(t: f64) -> Time {
        Time((t * PRECISION_F64) as u64)
    }
}

impl Add for Time {
    type Output = Time;
    fn add(self, rhs: Time) -> Time {
        Time(self.0 + rhs.0)
    }
}

impl Add<f64> for Time {
    type Output = Time;
    fn add(self, rhs: f64) -> Time {
        self + Time::from(rhs)
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, rhs: Time) {
        *self = *self + rhs;
    }
}

impl AddAssign<f64> for Time {
    fn add_assign(&mut self, rhs: f64) {
        *self = *self + rhs;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t00_03_21() {
        let t = Time::parse("3.21").unwrap();
        assert_eq!(&t.to_string(), "  : 3.21");
    }

    #[test]
    fn t00_43_21() {
        let t = Time::parse("43.21").unwrap();
        assert_eq!(&t.to_string(), "  :43.21");
    }

    #[test]
    fn t05_43_21() {
        let t = Time::parse("5:43.21").unwrap();
        assert_eq!(&t.to_string(), " 5:43.21");
    }

    #[test]
    fn t65_43_21() {
        let t = Time::parse("65:43.21").unwrap();
        assert_eq!(&t.to_string(), "65:43.21");
    }

    #[test]
    fn at_60s() {
        let t = Time::from(60.0);
        assert_eq!(&t.to_string(), " 1:00.00");
    }
}
