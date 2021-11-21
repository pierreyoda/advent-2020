use std::convert::TryFrom;

use anyhow::{anyhow, Result};

use advent_2020_common::run_with_scaffolding_integers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PasswordPolicy {
    mandatory_character: char,
    minimum_occurrences: u8,
    maximum_occurrences: u8,
    password: Option<String>,
}

impl PasswordPolicy {
    pub fn new(
        character: char,
        range_min_inclusive: u8,
        range_max_inclusive: u8,
        password: Option<String>,
    ) -> Self {
        Self {
            mandatory_character: character,
            minimum_occurrences: range_min_inclusive,
            maximum_occurrences: range_max_inclusive,
            password,
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(ref password) = self.password {
            self.validate_against(&password[..])
        } else {
            false
        }
    }

    pub fn validate_against(&self, against: &str) -> bool {
        let mut count = 0;
        for character in against.chars() {
            if character == self.mandatory_character {
                count += 1;
            }
        }

        self.minimum_occurrences <= count && count <= self.maximum_occurrences
    }
}

impl TryFrom<String> for PasswordPolicy {
    type Error = anyhow::Error;

    // TODO: avoid unwrapping
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let split: Vec<_> = value.split(": ").collect();

        if split.is_empty() {
            return Err(anyhow!(
                "Insufficient information for parsing PasswordPolicy"
            ));
        }

        let policy_components: Vec<_> = split.get(0).unwrap().split(' ').collect();
        if policy_components.len() != 2 {
            return Err(anyhow!("Invalid password policy"));
        }

        let requirements: Vec<_> = policy_components.get(0).unwrap().split('-').collect();
        let [range_min_inclusive, range_max_inclusive] = [
            requirements.get(0).unwrap().parse().unwrap(),
            requirements.get(1).unwrap().parse().unwrap(),
        ];

        let character = policy_components.get(1).unwrap().chars().next().unwrap();

        // (Optional) password
        // not very pretty; this is done to parse at once
        let password = if split.len() == 2 {
            Some(split.get(1).unwrap().to_string())
        } else {
            None
        };

        Ok(Self::new(
            character,
            range_min_inclusive,
            range_max_inclusive,
            password,
        ))
    }
}

fn main() -> Result<()> {
    // Part 1
    run_with_scaffolding_integers("day-2", b'\n', |inputs| -> Result<u16, anyhow::Error> {
        dbg!(&inputs);
        Ok(inputs.iter().fold(0, |count, input| {
            let strategy: PasswordPolicy = PasswordPolicy::try_from(input.to_string()).unwrap();
            count + if strategy.validate() { 1 } else { 0 }
        }))
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::PasswordPolicy;

    #[test]
    fn test_password_validation() {
        assert!(PasswordPolicy::new('a', 1, 3, None).validate_against("abcde"));
        assert!(!PasswordPolicy::new('a', 1, 3, None).validate_against("cdefg"));
        assert!(PasswordPolicy::new('c', 2, 9, None).validate_against("ccccccccc"));
    }

    #[test]
    fn test_password_policy_parsing() {
        let lines = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        let lines_validity = [true, false, true];

        assert_eq!(lines.len(), lines_validity.len());
        for (i, line) in lines.iter().enumerate() {
            let policy = PasswordPolicy::try_from(line.to_string()).unwrap();
            assert_eq!(policy.validate(), *lines_validity.get(i).unwrap());
        }
    }
}
