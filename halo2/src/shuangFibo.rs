/// the step to write a circuit
/// config + (compute, storage<fill table>, generate proof)+ verify
/// ****config****
/// define constraints. i.e., in meta.create_gate, query cell from table, and define the constraints from the queried cells
/// expression --> you can think it as a way to get a value from a cell
/// **** compute, storage****
/// 
use halo2_proofs::circuit::{self, *};
use halo2_proofs::arithmetic::Field;
use halo2_proofs::dev::MockProver;
use halo2_proofs::plonk::*;
use halo2_proofs::poly::Rotation;


#[derive(Clone, Debug, Copy)]
struct Config {
    elem_1: Column<Advice>,
    elem_2: Column<Advice>,
    elem_3: Column<Advice>,
    q_fib: Selector,
}

///in Config, there are three funtions:
/// configuration: set advice column, enable permutation colomns,create custom gates
///     defince column by                advice_column
///     enable equality: cs.             enable_equality
///     create custom gate using         cs.create_gate      in the virtualcell, query the cells using query_advice query_selector
/// init
/// assign

impl Config {
    fn configure<F:Field>(cs: &mut ConstraintSystem<F>)-> Self{ //&mut means that the function can modify the referenced ConstraintSystem<F> instance.
        let elem_1=cs.advice_column();
        let elem_2=cs.advice_column();
        let elem_3=cs.advice_column();
        let q_fib=cs.selector();

        cs.enable_equality(elem_1);
        cs.enable_equality(elem_2);
        cs.enable_equality(elem_3);

        cs.create_gate("add", |virtual_cells: &mut VirtualCells<'_,F>|{
            let q_fib=virtual_cells.query_selector(q_fib);
            let elem_1=virtual_cells.query_advice(elem_1, Rotation::cur());
            let elem_2=virtual_cells.query_advice(elem_2,Rotation::cur());
            let elem_3=virtual_cells.query_advice(elem_3, Rotation::cur());

            vec![
                q_fib * (elem_1+elem_2-elem_3),
            ]

        });
        Self { elem_1, elem_2, elem_3,q_fib }

    }
    fn init<F:Field>(
        &self,
        mut layouter: impl Layouter<F>, 
        elem_1: Value<F>,
        elem_2: Value<F>,
    ) -> Result<(
        AssignedCell<F, F>, // elem_2
        AssignedCell<F, F> // elem_3
    ), Error> {
        layouter.assign_region(||"init fibonacci", |mut region|{
            let offset=0;

            //enable q_fib
            self.q_fib.enable(&mut region, offset)?;

            //assign elem_1
            region.assign_advice(||"elem_1", self.elem_1, offset,||elem_1)?;

            //assign elem_2
            let elem_2=region.assign_advice(||"elem_2", self.elem_2, offset, ||elem_2)?;

            let elem_3=elem_1 + elem_2.value_field().evaluate();

            let elem_3 = region.assign_advice(||"elem_3", self.elem_3, offset, ||elem_3)?;

            Ok((
                elem_2,
                elem_3
            ))

        })
    

    }

    fn assign<F: Field> (
        &self,
        mut layouter: impl Layouter<F>, //mut layouter: impl Layouter<F>: This parameter is named layouter and is declared as mutable (mut). 
                                        //It's of a type that implements the Layouter<F> trait. The impl Layouter<F> syntax indicates that 
                                        //layouter can be of any type that implements the Layouter<F> trait.
        elem_2: AssignedCell<F,F>,
        elem_3: AssignedCell<F,F>,
    
    )-> Result<(
        AssignedCell<F,F>, //elem_2 AssignedCell is likely a generic type that takes two type parameters. The first F may represent the type of the value 
                        //contained within the cell, while the second F could represent additional properties or constraints associated with the cell.
        AssignedCell<F,F>// elem_3
    ),Error>{
        layouter.assign_region(||"steady-state Fibonacci", |mut region|{
            let offset=0;

            //enable q_fib
            self.q_fib.enable(&mut region, offset)?;

            //copy elem_1
            let elem_1=elem_2.copy_advice(||"copy elem_2 into current elem_1", &mut region, self.elem_1, offset)?;

            //copy elem_2 
            let elem_2=elem_3.copy_advice(||"copy elem_3 into current elem_2", &mut region, self.elem_2, offset)?;

            let elem_3=elem_1.value_field().evaluate()   +elem_2.value_field().evaluate();

            let elem_3=region.assign_advice(||"elem_3", self.elem_3, offset, || elem_3)?;

            Ok((
                elem_2,
                elem_3
            ))
        })


    }
}
//In Rust, #[cfg(test)] is an attribute used to conditionally compile code only when running tests. It is typically used to include 
//test-specific code that should not be included in the final binary produced by cargo build.
#[cfg(test)]

mod tests{
    use halo2_proofs::{circuit::SimpleFloorPlanner,pasta::Fp,dev::MockProver};

    use super::*;

#[derive(Default)]
struct MyCircuit<F:Field>{
    elem_1: Value<F>,
    elem_2: Value<F>,
}
// Circuit has there components:
// config, floorplaner
/// without witness
/// configure
/// synsized
impl <F:Field> Circuit<F> for MyCircuit<F>{
    type Config = Config;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        Self::Config::configure(meta)
    }

    fn synthesize(&self, config: Self::Config, mut layouter: impl Layouter<F>) -> Result<(), Error> {
        ///In Rust, the ? operator is used for error propagation. It is shorthand for a combination of match and return. When used, 
        /// it will automatically propagate the error upwards, similar to how exceptions work in other languages.

        let (elem_2,elem_3)=config.init(layouter.namespace(||"init"), self.elem_1,self.elem_2)?;

        config.assign(layouter.namespace(||"first assign after init"), elem_2, elem_3)?;
        ///This creates an Ok result containing a unit value (), indicating that the function execution was successful. It's a common pattern in 
        /// Rust to return Ok(()) at the end of a function to indicate success when using the ? operator for error handling.
        Ok(())
    }

}


#[test]
fn test_fib(){
    let circuit=MyCircuit{
      elem_1: Value::known(Fp::one()),
      elem_2: Value::known(Fp::one()),  
    };

    let prover = MockProver::run(3, &circuit, vec![]).unwrap();
    prover.assert_satisfied();
}
}