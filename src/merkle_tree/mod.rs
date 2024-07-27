// Merkle tree is a binary tree where each leaf node is a hash of a data block and each non-leaf node is a hash of its children.
// -> This is a wrapper around, rs_merkle crate.
// What hash function should be used?
// -> we are using SHA256 hash function provided by rs_merkle crate.
// should the hashes be in the prime field?
// -> not of the need.

use crate::field::FieldElement;
use rs_merkle::{algorithms::Sha256, Hasher, MerkleProof, MerkleTree as MerkleTreeTrait};

#[derive(Clone)]
pub struct MerkleTree {
    pub data: Vec<FieldElement>,
    pub leaves: Vec<[u8; 32]>,
    pub inner: MerkleTreeTrait<Sha256>,
}

impl MerkleTree {
    pub fn new(data: &[FieldElement]) -> MerkleTree {
        let leaves: Vec<[u8; 32]> = data
            .into_iter()
            .map(|x| Sha256::hash(&x.0.to_be_bytes()))
            .collect();
        let merkle_tree = MerkleTreeTrait::<Sha256>::from_leaves(&leaves);
        MerkleTree {
            data: data.to_vec(),
            leaves: leaves,
            inner: merkle_tree,
        }
    }

    pub fn get_authentication_path(&self, index: usize) -> Vec<u8> {
        self.leaves.get(index).unwrap();
        let proof = self.inner.proof(&[index]);
        proof.to_bytes()
    }

    pub fn validate(&self, proof_bytes: Vec<u8>, index: usize) -> bool {
        let proof = MerkleProof::<Sha256>::try_from(proof_bytes).unwrap();
        let merkle_root = self.inner.root().unwrap();
        let leaf_to_prove = self.leaves.get(index).unwrap();
        proof.verify(merkle_root, &[index], &[*leaf_to_prove], self.leaves.len())
    }
}

#[cfg(test)]
mod test_merkle_implementation {
    use crate::field::{Field, FieldElement};
    #[test]
    fn test_merkle_tree() {
        let field = Field::new(7);
        let data = vec![
            FieldElement::new(1, field),
            FieldElement::new(2, field),
            FieldElement::new(3, field),
            FieldElement::new(4, field),
            FieldElement::new(5, field),
            FieldElement::new(6, field),
        ];
        let merkle_tree = super::MerkleTree::new(&data);
        let proof = merkle_tree.get_authentication_path(0);
        assert_eq!(merkle_tree.validate(proof, 0), true);
    }
}
