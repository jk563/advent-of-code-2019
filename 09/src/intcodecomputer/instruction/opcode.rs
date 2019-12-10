#[derive(Debug)]
#[derive(PartialEq)]
pub enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    JumpTrue,
    JumpFalse,
    Less,
    Equal,
    RelativeBase,
    Halt,
}

impl Opcode {
    pub fn from(opcode: usize) -> Opcode {
        match opcode {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JumpTrue,
            6 => Opcode::JumpFalse,
            7 => Opcode::Less,
            8 => Opcode::Equal,
            9 => Opcode::RelativeBase,
            99 => Opcode::Halt,
            _ => panic!("Opcode not recognised: {}", opcode),
        }
    }

    pub fn parameter_count(&self) -> usize {
        match self {
            Opcode::Add | Opcode::Multiply | Opcode::Less | Opcode::Equal => 3,
            Opcode::JumpTrue | Opcode::JumpFalse => 2,
            Opcode::Input | Opcode::Output | Opcode::RelativeBase => 1,
            Opcode::Halt => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Opcode;

    #[test]
    fn test_opcode_from() {
        let opcode_from_1 = Opcode::from(1);
        let opcode_from_2 = Opcode::from(2);
        let opcode_from_3 = Opcode::from(3);
        let opcode_from_4 = Opcode::from(4);
        let opcode_from_5 = Opcode::from(5);
        let opcode_from_6 = Opcode::from(6);
        let opcode_from_7 = Opcode::from(7);
        let opcode_from_8 = Opcode::from(8);
        let opcode_from_9 = Opcode::from(9);
        let opcode_from_99 = Opcode::from(99);

        assert_eq!(opcode_from_1, Opcode::Add);
        assert_eq!(opcode_from_2, Opcode::Multiply);
        assert_eq!(opcode_from_3, Opcode::Input);
        assert_eq!(opcode_from_4, Opcode::Output);
        assert_eq!(opcode_from_5, Opcode::JumpTrue);
        assert_eq!(opcode_from_6, Opcode::JumpFalse);
        assert_eq!(opcode_from_7, Opcode::Less);
        assert_eq!(opcode_from_8, Opcode::Equal);
        assert_eq!(opcode_from_9, Opcode::RelativeBase);
        assert_eq!(opcode_from_99, Opcode::Halt);
    }

    #[test]
    fn test_number_of_parameter() {
        assert_eq!(Opcode::Add.parameter_count(), 3);
        assert_eq!(Opcode::Multiply.parameter_count(), 3);
        assert_eq!(Opcode::Input.parameter_count(), 1);
        assert_eq!(Opcode::Output.parameter_count(), 1);
        assert_eq!(Opcode::JumpTrue.parameter_count(), 2);
        assert_eq!(Opcode::JumpFalse.parameter_count(), 2);
        assert_eq!(Opcode::Less.parameter_count(), 3);
        assert_eq!(Opcode::Equal.parameter_count(), 3);
        assert_eq!(Opcode::Halt.parameter_count(), 0);
    }
}
