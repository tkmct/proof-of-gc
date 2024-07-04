use mpz_circuits::{trace, Circuit, CircuitBuilder};

#[trace]
fn bitxor(a: bool, b: bool) -> bool {
    a ^ b
}

fn xor_circ() -> Circuit {
    let builder = CircuitBuilder::new();
    let a = builder.add_input::<bool>();
    let b = builder.add_input::<bool>();
    let c = bitxor_trace(&mut builder.state(), a, b);
    builder.add_output(c);
    builder.build().unwrap()
}

#[trace]
fn bitand(a: bool, b: bool) -> bool {
    a & b
}

fn and_circ() -> Circuit {
    let builder = CircuitBuilder::new();
    let a = builder.add_input::<bool>();
    let b = builder.add_input::<bool>();
    let c = bitand_trace(&mut builder.state(), a, b);
    builder.add_output(c);
    builder.build().unwrap()
}

fn main() {
    // prove generated garbled circuit is correctly generated
    // garbler encode the wires using randomness and send it to generator

    // Firstly, let's try the simplest example: Prove single AND gate

    // XOR gate is free (Free-XOR technique)
    // we use shared delta to represent zero-label and on e-label is zero-label + delta

    // given boolean circuit
    // randomly sample delta for the GC
    // randomly sample zero labels for every input wires
    // randomly sample zero labels for every output gates for AND gate
    // For every and gate, construct half gate
    // Output decoding data
    let xor_circuit = xor_circ();
    let and_circuit = and_circ();
    println!("XOR_CIRC: {:?}", xor_circuit);
    println!("AND_CIRC: {:?}", and_circuit);
}
