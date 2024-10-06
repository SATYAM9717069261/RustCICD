use logicGate::{and, xor};

pub type Sum = u8; // alias type like #define
pub type Carry = u8;

pub fn half_adder_input_output() -> Vec<((u8, u8), (Sum, Carry))> {
    vec![
        ((0, 0), (0, 0)),
        ((0, 1), (1, 0)),
        ((1, 0), (1, 0)),
        ((1, 1), (0, 1)),
    ]
}

fn half_adder(num1: u8, num2: u8) -> (Sum, Carry) {
    (xor(num1, num2), and(num1, num2))
}

#[test]
fn one_bit_adder() {
    for (inn, out) in half_adder_input_output() {
        let (a, b) = inn;
        println!("Testing: {}, {} -> {}", a, b, out.0);
        assert_eq!(half_adder(a, b), out);
    }
}
