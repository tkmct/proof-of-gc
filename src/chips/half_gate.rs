///! Implement half gate chip to prove correct half gate construction
use crate::halo2_proofs::{circuit::AssignedCell, halo2curves::bn256::Fr as Fp, plonk::Error};

#[derive(Clone, Copy, Debug)]
pub struct HalfGateConfig {}

impl HalfGateConfig {}

#[derive(Clone, Copy, Debug)]
pub struct HalfGateChip {
    config: HalfGateConfig,
}

impl HalfGateChip {
    fn construct(config: HalfGateConfig) -> Self {
        Self { config }
    }

    pub fn configure() -> HalfGateConfig {
        HalfGateConfig {}
    }

    pub fn generate_half_gate() -> Result<AssignedCell<Fp, Fp>, Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::halo2_proofs::{
        circuit::{Layouter, SimpleFloorPlanner},
        halo2curves::bn256::Fr as Fp,
        plonk::{Circuit, ConstraintSystem},
    };

    use super::*;

    #[derive(Clone)]
    struct TestCircuit {
        x: String,
        y: String,
    }

    impl Circuit<Fp> for TestCircuit {
        type Config = HalfGateConfig;
        type FloorPlanner = SimpleFloorPlanner;

        fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
            todo!()
        }

        fn synthesize(
            &self,
            config: Self::Config,
            layouter: impl Layouter<Fp>,
        ) -> Result<(), Error> {
            todo!()
        }

        fn without_witnesses(&self) -> Self {
            todo!()
        }
    }
}
