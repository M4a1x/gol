use super::ParseError;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Rules {
    survive: Vec<u8>,
    birth: Vec<u8>,
}

impl Rules {
    pub fn new(survive: Vec<u8>, birth: Vec<u8>) -> Self {
        if !Self::is_valid_ruleset(&survive) {
            panic!(format!("Invalid ruleset: {:?}", survive));
        }

        if !Self::is_valid_ruleset(&birth) {
            panic!(format!("Invalid ruleset: {:?}", survive));
        }

        Rules { survive, birth }
    }

    fn is_valid_ruleset(ruleset: &[u8]) -> bool {
        ruleset.iter().filter(|&num| num > &8).count() == 0
    }

    pub fn get_birthrule(&self) -> &[u8] {
        &self.birth
    }

    pub fn get_surviverule(&self) -> &[u8] {
        &self.survive
    }
}

impl Default for Rules {
    fn default() -> Self {
        Rules {
            survive: vec![2, 3],
            birth: vec![3],
        }
    }
}

impl PartialEq for Rules {
    fn eq(&self, other: &Self) -> bool {
        if self.survive.len() != other.survive.len() {
            println!("Length not equal");
            return false;
        }
        if self.birth.len() != other.birth.len() {
            println!("Length not equal");
            return false;
        }

        let survive_equal = self
            .survive
            .iter()
            .filter(|r| !other.survive.contains(r))
            .count()
            == 0;

        let birth_equal = self
            .birth
            .iter()
            .filter(|r| !other.birth.contains(r))
            .count()
            == 0;

        birth_equal && survive_equal
    }
}
impl Eq for Rules {}

impl FromStr for Rules {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules: Vec<&str> = s.split('/').collect();
        if rules.len() != 2 {
            return Err(ParseError::InvalidFormat(s.into()));
        }

        fn parse_ruleset(s: &str) -> Result<Vec<u8>, ParseError> {
            let mut ruleset = s
                .chars()
                .map(|c| c.to_digit(10).map(|d| (d as u8)))
                .collect::<Option<Vec<_>>>()
                .ok_or_else(|| ParseError::InvalidDigit(s.into()))?;

            if ruleset.contains(&9) {
                return Err(ParseError::InvalidDigit("9".into()));
            }

            ruleset.sort_unstable();
            ruleset.dedup();
            Ok(ruleset)
        }

        let survive = parse_ruleset(rules[0])?;
        let birth = parse_ruleset(rules[1])?;

        Ok(Rules { survive, birth })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rules() {
        let r1 = "23/3";
        let r1_result = Rules {
            survive: vec![2, 3],
            birth: vec![3],
        };

        let r2 = "12378/432";
        let r2_result = Rules {
            survive: vec![1, 2, 3, 7, 8],
            birth: vec![4, 3, 2],
        };

        let r3 = "/";
        let r3_result = Rules {
            survive: vec![],
            birth: vec![],
        };

        assert_eq!(Rules::from_str(r1).unwrap(), r1_result);
        assert_eq!(Rules::from_str(r2).unwrap(), r2_result);
        assert_eq!(Rules::from_str(r3).unwrap(), r3_result);
    }

    #[test]
    fn test_parse_rules_invalid_digit() {
        let rule1 = Rules::from_str("2a/3");
        let rule2 = Rules::from_str("12/9");

        assert_eq!(rule1.unwrap_err(), ParseError::InvalidDigit("2a".into()));
        assert_eq!(rule2.unwrap_err(), ParseError::InvalidDigit("9".into()));
    }

    #[test]
    fn test_parse_rules_invalid_format() {
        let rule1 = Rules::from_str("23-3");
        let rule2 = Rules::from_str("1");

        assert_eq!(rule1.unwrap_err(), ParseError::InvalidFormat("23-3".into()));
        assert_eq!(rule2.unwrap_err(), ParseError::InvalidFormat("1".into()));
    }
}
