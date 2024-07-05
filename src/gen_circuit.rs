use crate::halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner},
    halo2curves::bn256::Fr as Fp,
    plonk::{Circuit, ConstraintSystem},
};

use mpz_circuits::Circuit as GcCircuit;
use mpz_garble_core::EncryptedGate;

pub struct GenConfig {}

impl GenConfig {
    pub fn configure(meta: &mut ConstraintSystem<Fp>) -> Self {
        // generate AES config depend on circuit size
        // AES calls == number of AND gates * 2 (half gates)
        todo!()
    }

    pub fn assign(&mut self) {
        todo!()
    }
}

#[derive(Clone)]
pub struct GenCircuit {
    gc_circuit: GcCircuit,
    encrypted_gates: Vec<EncryptedGate>,
}

impl GenCircuit {
    pub fn new(gc_circuit: GcCircuit, encrypted_gates: Vec<EncryptedGate>) -> Self {
        Self {
            gc_circuit,
            encrypted_gates,
        }
    }
}

impl Circuit<Fp> for GenCircuit {
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
