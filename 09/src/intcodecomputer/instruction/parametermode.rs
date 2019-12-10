#[derive(Debug)]
#[derive(PartialEq)]
pub enum ParameterMode {
    PositionalMode,
    ImmediateMode,
    RelativeMode,
}

impl ParameterMode {
    pub fn from(input: usize) -> ParameterMode {
        match input {
            0 => ParameterMode::PositionalMode,
            1 => ParameterMode::ImmediateMode,
            2 => ParameterMode::RelativeMode,
            _ => panic!("ParameterMode not implemented: {}", input),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(ParameterMode::from(0), ParameterMode::PositionalMode);
        assert_eq!(ParameterMode::from(1), ParameterMode::ImmediateMode);
        assert_eq!(ParameterMode::from(2), ParameterMode::RelativeMode);
    }
}
