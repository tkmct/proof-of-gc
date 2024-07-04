use halo2_aes::halo2_proofs::dev::MockProver;
use mpz_circuits::{Circuit, CircuitBuilder};
use mpz_garble_core::{ChaChaEncoder, Encoder, Generator};

use proof_of_gc::GenCircuit;

fn and_circ() -> Circuit {
    let builder = CircuitBuilder::new();
    let a = builder.add_input::<bool>();
    let b = builder.add_input::<bool>();
    let c = a & b;
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
    // let xor_circuit = xor_circ();
    // println!("XOR_CIRC: {:?}", xor_circuit);

    let and_circuit = and_circ();
    println!("AND_CIRC: {:?}", and_circuit);

    // garbler.garble(xor_circuit);
    let encoder = ChaChaEncoder::new([0; 32]);
    let inputs = and_circuit
        .inputs()
        .iter()
        .map(|input| encoder.encode_by_type(0, &input.value_type()))
        .collect::<Vec<_>>();
    let mut gen = Generator::default();

    let mut gate_iter = gen.generate(&and_circuit, encoder.delta(), inputs).unwrap();
    let enc_gates = gate_iter.by_ref().collect::<Vec<_>>();

    println!("Gates: {:?}", enc_gates);

    // Prove garbled circuit generation
    let k = 20;
    let gen_circ = GenCircuit::new(and_circuit, enc_gates);
    let mock_prover = MockProver::run(k, &gen_circ, vec![]).unwrap();
    mock_prover.assert_satisfied();

    // garbled circuit evaluation
}
