#[derive(Debug, Clone)]
pub enum GateType {
    And,    // multi-input
    Or,     // multi-input
    Equal,  // 2-input
    Bigger, // 2-input (A > B)
}

#[derive(Debug, Clone)]
pub struct Gate {
    pub gate_type: GateType,
    pub input_gates_indices: Vec<usize>, // Which previous gates to use
    pub input_bits_indices: Vec<usize>,  // Which input bits to use
}

impl Gate {
    pub fn eval(&self, input_bits: &Vec<bool>, evaluated_gates: &Vec<bool>) -> bool {
        //TODO: checks
        let mut extracted_bits: Vec<bool> = Vec::new();
        for index_ref in &self.input_bits_indices {
            let index = *index_ref;
            assert_ne!(index >= input_bits.len(), true);
            extracted_bits.push(input_bits[index]);
        }

        for index_ref in &self.input_gates_indices {
            let index = *index_ref;
            assert_ne!(index >= evaluated_gates.len(), true);
            extracted_bits.push(evaluated_gates[index]);
        }

        assert_ne!(extracted_bits.len(), 0);

        match self.gate_type {
            GateType::And => return self.eval_and(&extracted_bits),
            GateType::Or => return self.eval_or(&extracted_bits),
            GateType::Equal => return self.eval_equal(&extracted_bits),
            GateType::Bigger => return self.eval_bigger(&extracted_bits),
        }
    }

    fn eval_and(&self, input_bits: &Vec<bool>) -> bool {
        assert_ne!(input_bits.len(), 0);
        for bit in input_bits.iter() {
            if !(*bit) {
                return false;
            }
        }
        return true;
    }

    fn eval_or(&self, input_bits: &Vec<bool>) -> bool {
        assert_ne!(input_bits.len(), 0);
        for bit in input_bits.iter() {
            if *bit {
                return true;
            }
        }
        return false;
    }

    fn eval_equal(&self, input_bits: &Vec<bool>) -> bool {
        assert_eq!(input_bits.len(), 2);
        return input_bits[0] == input_bits[1];
    }

    fn eval_bigger(&self, input_bits: &Vec<bool>) -> bool {
        assert_eq!(input_bits.len(), 2);
        return input_bits[0] && !input_bits[1];
    }
}
