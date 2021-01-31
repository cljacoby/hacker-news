use std::error::Error;

#[derive(Debug, PartialEq)]
struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self {
            year,
            month,
            day
        }
    }

    pub fn parse(date: String) -> Result<Self, Box<dyn Error>> {
        let mut iter = date.split("-");
        let year = iter.next()
            .ok_or("Failed to parse text date at year")?
            .parse::<u16>()?;
        let month = iter.next()
            .ok_or("Failed to parse text date at month")?
            .parse::<u8>()?;
        let day = iter.next()
            .ok_or("Failed to parse text date at day")?
            .parse::<u8>()?;

        Ok(Self {year, month, day})
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_date() -> Result<(), Box<dyn Error>> {
        let test = (
            "2020-02-29".to_string(),
            Date { year: 2020, month: 2, day: 29 }
        );
        assert_eq!(Date::parse(test.0)?, test.1);

        Ok(())
    }
}

