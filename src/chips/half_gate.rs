///! Implement half gate chip to prove correct half gate construction
use crate::{
    chips::{
        u8_and_chip::{U8AndChip, U8AndConfig},
        u8_xor_chip::{U8XorChip, U8XorConfig},
    },
    halo2_proofs::{
        circuit::{AssignedCell, Layouter, Value},
        halo2curves::bn256::Fr as Fp,
        plonk::{Advice, Column, ConstraintSystem, Error, Fixed, Selector},
    },
};

#[derive(Clone, Copy, Debug)]
pub struct HalfGateConfig {
    advice: Column<Advice>,
    fixed: Column<Fixed>,
    q: Selector,

    u8_xor_config: U8XorConfig,
    u8_and_config: U8AndConfig,
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
        fixed: Column<Fixed>,
        selector: Selector,
        u8_xor_config: U8XorConfig,
        u8_and_config: U8AndConfig,
    ) -> HalfGateConfig {
        // TODO: setup custom gates for these.
        meta.enable_equality(advice);

        HalfGateConfig {
            advice,
            fixed,
            q: selector,
            u8_and_config,
            u8_xor_config,
        }
    }

    pub fn generate_half_gate(
        &self,
        layouter: &mut impl Layouter<Fp>,
        x_0: &[AssignedCell<Fp, Fp>],
        y_0: &[AssignedCell<Fp, Fp>],
        delta: &[AssignedCell<Fp, Fp>],
        gid: u128,
    ) -> Result<Vec<AssignedCell<Fp, Fp>>, Error> {
        // Assert label and delta length.
        // Block is [u8;16]. So it uses 16 cells -> 128bit values
        // Label, Delta are both Block
        assert_eq!(x_0.len(), 16);
        assert_eq!(y_0.len(), 16);
        assert_eq!(delta.len(), 16);

        let xor_chip = U8XorChip::construct(self.config.u8_xor_config);
        let and_chip = U8AndChip::construct(self.config.u8_and_config);

        let (gid_0, gid_1) = layouter.assign_region(
            || "Assign gid_0, gid_1 as block",
            |mut region| {
                let mut offset = 0;

                // Assign gid in fixed column
                let gid_0 = gid
                    .to_be_bytes()
                    .iter()
                    .map(|b| {
                        let res = region.assign_fixed(
                            || "Assign gid for gate",
                            self.config.fixed,
                            offset,
                            || Value::known(Fp::from(*b as u64)),
                        );
                        offset += 1;
                        res
                    })
                    .collect::<Result<Vec<_>, Error>>()?;
                let gid_1 = (gid + 1)
                    .to_be_bytes()
                    .iter()
                    .map(|b| {
                        let res = region.assign_fixed(
                            || "Assign gid for gate",
                            self.config.fixed,
                            offset,
                            || Value::known(Fp::from(*b as u64)),
                        );
                        offset += 1;
                        res
                    })
                    .collect::<Result<Vec<_>, Error>>()?;

                Ok((gid_0, gid_1))
            },
        )?;

        // need u8 ^ u8 lookup table
        // let x_1 = x_0 ^ delta;
        // let y_1 = y_0 ^ delta;

        let x_1 = x_0
            .iter()
            .zip(delta)
            .map(|(x, d)| xor_chip.xor(layouter, &x, &d))
            .collect::<Result<Vec<AssignedCell<Fp, Fp>>, Error>>()?;

        // get color bit of x_0 and y_0.
        // let color_x = x_0.lsb();
        // let color_y = y_0.lsb();

        // let p_a = x_0.lsb();
        // let p_b = y_0.lsb();

        // let j = Block::new((gid as u128).to_be_bytes());
        // let k = Block::new(((gid + 1) as u128).to_be_bytes());

        // let mut h = [x_0, y_0, x_1, y_1];

        // Tweakable circular correlation-robosut hash function using fixed-key AES
        // Do 4 hashing here
        // cipher.tccr_many(&[j, k, j, k], &mut h);

        // let [hx_0, hy_0, hx_1, hy_1] = h;

        // // Garbled row of generator half-gate
        // let t_g = hx_0 ^ hx_1 ^ (Block::SELECT_MASK[color_y] & delta);
        // let w_g = hx_0 ^ (Block::SELECT_MASK[color_x] & t_g);

        // // Garbled row of evaluator half-gate
        // let t_e = hy_0 ^ hy_1 ^ x_0;
        // let w_e = hy_0 ^ (Block::SELECT_MASK[p_b] & (t_e ^ x_0));

        // let z_0 = Label::new(w_g ^ w_e);

        layouter.assign_region(
            || "Half-gate",
            |mut region| {
                let mut offset = 0;
                self.config.q.enable(&mut region, offset)?;

                // Copy x zero label to this region
                x_0.iter()
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

                Ok(vec![])
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::halo2_proofs::{
        circuit::{Layouter, SimpleFloorPlanner, Value},
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
        gid: u128,
    }

    impl Circuit<Fp> for TestCircuit {
        type Config = HalfGateConfig;
        type FloorPlanner = SimpleFloorPlanner;

        fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
            let advice = meta.advice_column();
            let fixed = meta.fixed_column();
            let selector = meta.selector();

            let q_xor = meta.complex_selector();
            let q_and = meta.complex_selector();
            let x = meta.advice_column();
            let y = meta.advice_column();
            let z = meta.advice_column();
            let tab_0 = meta.lookup_table_column();
            let tab_1 = meta.lookup_table_column();
            let tab_2 = meta.lookup_table_column();

            let u8_xor_config = U8XorChip::configure(meta, x, y, z, q_xor, tab_0, tab_1, tab_2);
            let u8_and_config = U8AndChip::configure(meta, x, y, z, q_and, tab_0, tab_1, tab_2);

            HalfGateChip::configure(meta, advice, fixed, selector, u8_xor_config, u8_and_config)
        }

        fn synthesize(
            &self,
            config: Self::Config,
            mut layouter: impl Layouter<Fp>,
        ) -> Result<(), Error> {
            let chip = HalfGateChip::construct(config);

            // let (gid_0, gid_1) = layouter.assign_region(
            //     || "Assign gid_0, gid_1 as block",
            //     |mut region| {
            //         let mut offset = 0;
            //         // Assign gid in fixed column
            //         let gid_0 = self
            //             .gid
            //             .to_be_bytes()
            //             .iter()
            //             .map(|b| {
            //                 region.assign_fixed(
            //                     || "Assign gid for gate",
            //                     config.fixed,
            //                     offset,
            //                     || Value::known(Fp::from(*b as u64)),
            //                 )
            //             })
            //             .collect::<Result<Vec<_>, Error>>()?;
            //         let gid_1 = (self.gid + 1)
            //             .to_be_bytes()
            //             .iter()
            //             .map(|b| {
            //                 region.assign_fixed(
            //                     || "Assign gid for gate",
            //                     config.fixed,
            //                     offset,
            //                     || Value::known(Fp::from(*b as u64)),
            //                 )
            //             })
            //             .collect::<Result<Vec<_>, Error>>()?;

            //         Ok((gid_0, gid_1))
            //     },
            // )?;

            // Assign x and y to advice column
            let (x_label, y_label, delta) = layouter.assign_region(
                || "Assign values",
                |mut region| {
                    let mut offset = 0;

                    let x_label = self
                        .x
                        .to_bytes()
                        .iter()
                        .map(|byte| {
                            let res = region.assign_advice(
                                || "assign x label",
                                config.advice,
                                offset,
                                || Value::known(Fp::from(*byte as u64)),
                            );
                            offset += 1;
                            res
                        })
                        .collect::<Result<Vec<_>, Error>>()?;
                    let y_label = self
                        .y
                        .to_bytes()
                        .iter()
                        .map(|byte| {
                            let res = region.assign_advice(
                                || "assign y label",
                                config.advice,
                                offset,
                                || Value::known(Fp::from(*byte as u64)),
                            );
                            offset += 1;
                            res
                        })
                        .collect::<Result<Vec<_>, Error>>()?;

                    let delta = self
                        .delta
                        .to_bytes()
                        .iter()
                        .map(|byte| {
                            let res = region.assign_advice(
                                || "assign delat",
                                config.advice,
                                offset,
                                || Value::known(Fp::from(*byte as u64)),
                            );
                            offset += 1;
                            res
                        })
                        .collect::<Result<Vec<_>, Error>>()?;

                    Ok((x_label, y_label, delta))
                },
            )?;

            let out =
                chip.generate_half_gate(&mut layouter, &x_label, &y_label, &delta, self.gid)?;

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
            gid: 0,
        };

        let mock = MockProver::run(k, &circ, vec![]).unwrap();
        mock.assert_satisfied();
    }
}
