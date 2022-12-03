use super::{Challenge, ChallengeError};
use std::fs;

pub struct Day01;

impl Day01 {
    const FILE_PATH: &'static str = "Input/day_01.txt";

    fn first_from_string(&self, data: &String) -> Result<String, ChallengeError> {
        let mut most = 0;
        let mut current = 0;

        for line in data.lines() {
            if line.is_empty() {
                if current > most {
                    most = current;
                }
                current = 0;
            } else {
                current += line
                    .parse::<u32>()
                    .map_err(|_| ChallengeError::InvalidInput)?;
            }
        }

        // Check corner case where last line wasn't empty
        if current > most {
            most = current;
        }

        Ok(most.to_string())
    }
}

impl Challenge for Day01 {
    fn first(&self) -> Result<String, ChallengeError> {
        let data = fs::read_to_string(Self::FILE_PATH).map_err(|_| ChallengeError::FileNotFound)?;
        self.first_from_string(&data)
    }

    fn second(&self) -> Result<String, ChallengeError> {
        Err(ChallengeError::NoAnswer)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_first() {
        let day = Day01;
        let data = "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
            .to_string();

        assert_eq!(day.first_from_string(&data).unwrap(), "24000");
    }
}
