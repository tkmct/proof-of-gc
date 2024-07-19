use crate::halo2_proofs::{
    circuit::{AssignedCell, Layouter, Value},
    halo2curves::bn256::Fr as Fp,
    plonk::{Advice, Column, ConstraintSystem, Error, Selector, TableColumn},
    poly::Rotation,
};

#[derive(Clone, Copy, Debug)]
pub struct U8AndConfig {
    x: Column<Advice>,
    y: Column<Advice>,
    z: Column<Advice>,
    q: Selector,
}

#[derive(Clone, Copy, Debug)]
pub struct U8AndChip {
    config: U8AndConfig,
}

impl U8AndChip {
    pub fn construct(config: U8AndConfig) -> Self {
        Self { config }
    }

    pub fn configure(
        meta: &mut ConstraintSystem<Fp>,
        x_col: Column<Advice>,
        y_col: Column<Advice>,
        z_col: Column<Advice>,
        selector: Selector,
        x_tab: TableColumn,
        y_tab: TableColumn,
        z_tab: TableColumn,
    ) -> U8AndConfig {
        meta.lookup("Check correct XOR of u8 values", |meta| {
            let q = meta.query_selector(selector);
            let x = meta.query_advice(x_col, Rotation::cur());
            let y = meta.query_advice(y_col, Rotation::cur());
            let z = meta.query_advice(z_col, Rotation::cur());

            vec![
                (q.clone() * x, x_tab),
                (q.clone() * y, y_tab),
                (q * z, z_tab),
            ]
        });

        U8AndConfig {
            x: x_col,
            y: y_col,
            z: z_col,
            q: selector,
        }
    }

    pub fn and(
        &self,
        layouter: &mut impl Layouter<Fp>,
        x: &AssignedCell<Fp, Fp>,
        y: &AssignedCell<Fp, Fp>,
    ) -> Result<AssignedCell<Fp, Fp>, Error> {
        layouter.assign_region(
            || "",
            |mut region| {
                self.config.q.enable(&mut region, 0)?;
                let x_copied = x.copy_advice(
                    || "assign x value to check u8 and",
                    &mut region,
                    self.config.x,
                    0,
                )?;
                let y_copied = y.copy_advice(
                    || "assign y value to check u8 and",
                    &mut region,
                    self.config.y,
                    0,
                )?;
                let z = region.assign_advice(
                    || "assign z value to check u8 and",
                    self.config.z,
                    0,
                    || {
                        and_bytes(
                            &x_copied.value_field().evaluate(),
                            &y_copied.value_field().evaluate(),
                        )
                    },
                );

                Ok(z)
            },
        )?
    }
}

/// Calculate and of given two bytes.
/// Returns the new value
pub(crate) fn and_bytes(x: &Value<Fp>, y: &Value<Fp>) -> Value<Fp> {
    // x and y should be u8.
    x.zip(*y)
        .map(|(x, y)| {
            x.to_bytes()
                .iter()
                .zip(y.to_bytes())
                .map(|(x_b, y_b)| x_b & y_b)
                .collect::<Vec<_>>()
        })
        .map(|bytes| Fp::from_bytes(&bytes.try_into().unwrap()).unwrap())
}
