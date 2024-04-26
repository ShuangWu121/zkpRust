use core::num;
use std::thread::Builder;

use plonky2::{field::types::Field, gates::public_input, hash::{hash_types::{HashOutTarget, MerkleCapTarget}, merkle_proofs::MerkleProofTarget, poseidon::PoseidonHash}, iop::{target::Target, witness::PartialWitness}, plonk::circuit_builder::CircuitBuilder};
pub struct SemaphoreTargets{
    merkle_root: HashOutTarget,
    topic: [Target;4],
    merkle_proof: MerkleProofTarget,
    private_key:[Target;4],
    public_key_index: Target,
}

impl AccessSet {
    pub fn tree_height(&self)->usize{
        
    }
    pub fn semaphore_circuit(&self,builder: &mut CircuitBuilder<F,2>)->SemaphoreTargets{
        //register public inputs
        let merkle_root=builder.add_virtual_hash();
        builder.register_public_input(&merkle_root.elements);
        let nullifier=builder.add_virtual_hash();
        builder.register_public_input(&nullifier.elements);
        
        let topic:[Target;4]=builder.add_virtual_targets(4).try_into().unwrap();
        builder.register_public_input(&topic);

        //merkle proof
        let merkle_proof=MerkleProofTarget{
            siblings:builder.add_virtual_hashes(self.tree_height()),
        };

        //verify public merkle treee proof
        let private_key:[Target;4]=builder.add_virtual_targets(4).try_into().unwrap();
        let public_key_index=builder.add_virtual_target();
        let public_key_index_bits=builder.split_le(public_key_index, self.tree_height());
        let zero=builder.zero();
        builder.verify_merkle_proof::<PoseidonHash>([private_key,[zero;4]].concat(),&public_key_index_bits,&MerkleCapTarget(vec![merkle_root]),&merkle_proof);

        //check nullifier
        let should_be_nullifier=builder.hash_n_to_hash_no_pad([private_key,topic].concat());
        for i in 0..4 {
            builder.connect(nullifier.elements[i], should_be_nullifier.elements[i]);
        }
        SemaphoreTargets {
            merkle_root,
            topic,
            merkle_proof,
            private_key,
            public_key_index,
        }

    }
    pub fn fill_semaphore_targets(&self,pw:&mut PartialWitness<F>,private_key:Digest,topic:Digest, public_key_index:usize,targets:SemaphoreTargets){
        

    }
    
}