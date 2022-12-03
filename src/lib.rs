pub mod day_01;

pub trait Challenge {
    fn first(&self) -> Result<String, ChallengeError>;
    fn second(&self) -> Result<String, ChallengeError>;
}

#[derive(Debug)]
pub enum ChallengeError {
    FileNotFound,
    InvalidInput,
    NoAnswer,
}

impl TryFrom<&String> for &dyn Challenge {
    type Error = ChallengeError;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value
            .parse::<u32>()
            .map_err(|_| ChallengeError::InvalidInput)?
        {
            1 => Ok(&day_01::Day01),
            _ => Err(ChallengeError::InvalidInput),
        }
    }
}
