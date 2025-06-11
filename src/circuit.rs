use crate::gate::*;

#[derive(Debug, Clone)]
pub struct Circuit {
    pub input_bits: Vec<bool>, // Input wires
    pub gates: Vec<Gate>,      // All gates
}

impl Circuit {
    pub fn eval(&self) -> bool {
        let mut gate_outputs: Vec<bool> = Vec::new();
        for gate in &self.gates {
            gate_outputs.push(gate.eval(&self.input_bits, &gate_outputs));
        }
        assert_ne!(gate_outputs.len(), 0);
        return gate_outputs[gate_outputs.len() - 1];
    }

    pub fn compare_n_bit_numbers(input_bits: Vec<bool>, n: usize) -> Self {
        assert_eq!(input_bits.len(), 2 * n);
        let mut gates: Vec<Gate> = Vec::new();
        for i in 0..n {
            let greater_gate: Gate = Gate {
                gate_type: GateType::Bigger,
                input_gates_indices: [].to_vec(),
                input_bits_indices: [(n - 1) - i, 2 * n - 1 - i].to_vec(),
            };

            gates.push(greater_gate);

            if i == 0 {
                continue;
            }

            let eq_gate: Gate = Gate {
                gate_type: GateType::Equal,
                input_gates_indices: [].to_vec(),
                input_bits_indices: [(n - 1) - i, 2 * n - 1 - i].to_vec(),
            };
            gates.push(eq_gate);

            let and_gate: Gate = Gate {
                gate_type: GateType::And,
                input_gates_indices: [gates.len() - 1, gates.len() - 3].to_vec(),
                input_bits_indices: [].to_vec(),
            };

            gates.push(and_gate);

            let or_gate: Gate = Gate {
                gate_type: GateType::Or,
                input_gates_indices: [gates.len() - 1, gates.len() - 3].to_vec(),
                input_bits_indices: [].to_vec(),
            };

            gates.push(or_gate);
        }

        return Circuit {
            input_bits: input_bits,
            gates: gates,
        };
    }
}
