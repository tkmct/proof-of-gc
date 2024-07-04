use halo2_aes::halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner},
    halo2curves::bn256::Fr as Fp,
    plonk::{Circuit, ConstraintSystem},
};

use mpz_circuits::Circuit as GcCircuit;
use mpz_garble_core::{encoding_state::Active, EncodedValue, EncryptedGate};

#[derive(Clone)]
pub struct EvalCircuit {
    gc_circuit: GcCircuit,
    encrypted_gates: Vec<EncryptedGate>,

    // List of wire labels
    inputs: Vec<EncodedValue<Active>>,
}

impl EvalCircuit {
    pub fn new(
        gc_circuit: GcCircuit,
        encrypted_gates: Vec<EncryptedGate>,
        inputs: Vec<EncodedValue<Active>>,
    ) -> Self {
        Self {
            gc_circuit,
            encrypted_gates,
            inputs,
        }
    }
}

impl Circuit<Fp> for EvalCircuit {
    type Config = ();
    type FloorPlanner = SimpleFloorPlanner;

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        todo!()
    }

    fn synthesize(
        &self,
        config: Self::Config,
        layouter: impl Layouter<Fp>,
    ) -> Result<(), halo2_aes::halo2_proofs::plonk::Error> {
        todo!()
    }

    fn without_witnesses(&self) -> Self {
        unimplemented!()
    }
}
