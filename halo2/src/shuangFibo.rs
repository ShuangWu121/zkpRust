/// the step to write a circuit
/// config + (compute, storage<fill table>, generate proof)+ verify
/// ****config****
/// define constraints. i.e., in meta.create_gate, query cell from table, and define the constraints from the queried cells
/// expression --> you can think it as a way to get a value from a cell
/// **** compute, storage****
/// 
use halo2_proofs::circuit::*;
use halo2_proofs::arithmetic::Field;
use halo2_proofs::plonk::*;


#[derive(Clone, Debug, Copy)]
struct Config {
    elem_1: Column<Advice>,
    elem_2: Column<Advice>,
    elem_3: Column<Advice>,
    q_fib: Selector,
}