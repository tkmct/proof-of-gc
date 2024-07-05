///! Implement half gate chip to prove correct half gate construction
use crate::halo2_proofs::{
    circuit::{AssignedCell, Layouter},
    halo2curves::bn256::Fr as Fp,
    plonk::{Advice, Column, ConstraintSystem, Error, Selector},
};

#[derive(Clone, Copy, Debug)]
pub struct HalfGateConfig {
    advice: Column<Advice>,
    q: Selector,
}

impl HalfGateConfig {}

#[derive(Clone, Copy, Debug)]
pub struct HalfGateChip {
    config: HalfGateConfig,
}

impl HalfGateChip {
    fn construct(config: HalfGateConfig) -> Self {
        Self { config }
    }

    pub fn configure(
        meta: &mut ConstraintSystem<Fp>,
        advice: Column<Advice>,
        selector: Selector,
    ) -> HalfGateConfig {
        // TODO: setup custom gates for these.

        HalfGateConfig {
            advice,
            q: selector,
        }
    }

    pub fn generate_half_gate(
        &self,
        layouter: &mut impl Layouter<Fp>,
        x_label: &[AssignedCell<Fp, Fp>],
        y_label: &[AssignedCell<Fp, Fp>],
        delta: &[AssignedCell<Fp, Fp>],
    ) -> Result<Vec<AssignedCell<Fp, Fp>>, Error> {
        // Assert label and delta length.
        // Block is [u8;16]. So it uses 16 cells -> 128bit values
        // Label, Delta are both Block
        assert_eq!(x_label.len(), 16);
        assert_eq!(y_label.len(), 16);
        assert_eq!(delta.len(), 16);

        layouter.assign_region(
            || "Half-gate",
            |mut region| {
                let mut offset = 0;
                self.config.q.enable(&mut region, offset)?;

                // Copy x zero label to this region
                x_label
                    .iter()
                    .map(|x| {
                        let cell = x.copy_advice(
                            || "Copy X label",
                            &mut region,
                            self.config.advice,
                            offset,
                        );
                        offset += 1;
                        cell
                    })
                    .collect::<Result<Vec<_>, Error>>()?;

                // NOTE: make hash function swappable
                // TODO: handle tccr
                // Tweakable circular correlation-robosut hash function using fixed-key AES

                Ok(vec![])
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::halo2_proofs::{
        circuit::{Layouter, SimpleFloorPlanner},
        dev::MockProver,
        halo2curves::bn256::Fr as Fp,
        plonk::{Circuit, ConstraintSystem},
    };
    use mpz_core::block::Block;

    #[derive(Clone)]
    struct TestCircuit {
        x: Block,
        y: Block,
        delta: Block,
    }

    impl Circuit<Fp> for TestCircuit {
        type Config = HalfGateConfig;
        type FloorPlanner = SimpleFloorPlanner;

        fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
            let advice = meta.advice_column();
            let selector = meta.selector();

            HalfGateChip::configure(meta, advice, selector)
        }

        fn synthesize(
            &self,
            config: Self::Config,
            layouter: &mut impl Layouter<Fp>,
        ) -> Result<(), Error> {
            let chip = HalfGateChip::construct(config);

            let out = chip.generate_half_gate(&mut layouter, x_label, y_label, delta)?;

            Ok(())
        }

        fn without_witnesses(&self) -> Self {
            todo!()
        }
    }

    #[test]
    fn test_half_gate_chip() {
        let k = 18;
        let circ = TestCircuit {
            x: Block::new([0u8; 16]),
            y: Block::new([1u8; 16]),
            delta: Block::new([2u8; 16]),
        };

        let mock = MockProver::run(k, &circ, vec![]).unwrap();
        mock.assert_satisfied();
    }
}
