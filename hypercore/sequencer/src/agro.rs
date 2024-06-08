// This file is part of Gear.
//
// Copyright (C) 2024 Gear Technologies Inc.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Abstract commitment aggregator.

use gprimitives::H256;
use hypercore_signer::{hash, Address, Signature};
use std::collections::HashMap;

pub trait SeqHash {
    fn hash(&self) -> H256;
}

#[derive(Clone, Debug)]
pub struct AggregatedCommitments<D: SeqHash> {
    pub commitments: Vec<D>,
    pub signature: Signature,
}

#[derive(Debug)]
pub struct LinkedAggregation<D: SeqHash> {
    pub aggregated: AggregatedCommitments<D>,
    pub previous: Option<H256>,
}

#[derive(Debug)]
pub struct AggregatedQueue<D: SeqHash> {
    all_commitments: HashMap<H256, LinkedAggregation<D>>,
    last: H256,
}

#[derive(Debug, Clone)]
pub struct CodeHashCommitment(H256);

// identity hashing
impl SeqHash for CodeHashCommitment {
    fn hash(&self) -> H256 {
        self.0
    }
}

impl<D: SeqHash> AggregatedCommitments<D> {
    pub fn hash(&self) -> H256 {
        let mut array = Vec::new();
        for commitment in &self.commitments {
            array.extend_from_slice(commitment.hash().as_ref());
        }
        hash(&array)
    }
}

impl<D: SeqHash> AggregatedQueue<D> {
    pub fn new(initial: AggregatedCommitments<D>) -> Self {
        let hash = initial.hash();
        let mut all_commitments = HashMap::new();
        all_commitments.insert(
            hash,
            LinkedAggregation {
                aggregated: initial,
                previous: None,
            },
        );
        Self {
            all_commitments,
            last: hash,
        }
    }

    pub fn push(&mut self, commitment: AggregatedCommitments<D>) {
        let hash = commitment.hash();
        self.last = hash;

        let new_queue = LinkedAggregation {
            aggregated: commitment,
            previous: Some(self.last),
        };

        self.all_commitments.insert(hash, new_queue);
    }

    pub fn get_signature(&self, commitment: H256) -> Option<Signature> {
        self.all_commitments
            .get(&commitment)
            .map(|c| c.aggregated.signature.clone())
    }

    pub fn previous(&self, commitment: H256) -> Option<H256> {
        self.all_commitments
            .get(&commitment)
            .and_then(|c| c.previous)
    }
}

#[derive(Clone, Debug)]
pub struct MultisignedCommitments<D> {
    pub commitments: Vec<D>,
    pub sources: Vec<Address>,
    pub signatures: Vec<Signature>,
}

pub struct Aggregator<D: SeqHash + Clone> {
    threshold: usize,

    aggregated: HashMap<Address, AggregatedQueue<D>>,
    plain_commitments: HashMap<H256, Vec<D>>,

    rolling: Option<H256>,
}

impl<D: SeqHash + Clone> Aggregator<D> {
    pub fn new(threshold: usize) -> Self {
        Self {
            threshold,
            aggregated: HashMap::new(),
            plain_commitments: HashMap::new(),
            rolling: None,
        }
    }

    pub fn push(&mut self, origin: Address, aggregated: AggregatedCommitments<D>) {
        let hash = aggregated.hash();

        self.plain_commitments
            .insert(hash, aggregated.commitments.clone());

        self.aggregated
            .entry(origin)
            .and_modify(|q| {
                q.push(aggregated.clone());
            })
            .or_insert_with(move || AggregatedQueue::new(aggregated));

        self.rolling = Some(hash);
    }

    pub fn len(&self) -> usize {
        self.aggregated.len()
    }

    pub fn find_root(self) -> Option<MultisignedCommitments<D>> {
        use std::collections::VecDeque;

        // Start only with the root
        let mut candidates = VecDeque::new();
        candidates.push_back(self.rolling?);

        while let Some(candidate_hash) = candidates.pop_front() {
            // check if we can find `threshold` amount of this `candidate_hash`
            let mut sources = vec![];
            let mut signatures = vec![];

            for (source, queue) in self.aggregated.iter() {
                if let Some(signature) = queue.get_signature(candidate_hash) {
                    sources.push(*source);
                    signatures.push(signature.clone());
                }
            }

            if signatures.len() >= self.threshold {
                // found our candidate
                let plain_commitments = self.plain_commitments.get(&candidate_hash)
                    .expect("Plain commitments should be present, as they are always updated when Aggregator::push is invoked; qed");

                let multi_signed = MultisignedCommitments {
                    commitments: plain_commitments.clone(),
                    sources,
                    signatures,
                };

                return Some(multi_signed);
            }

            // else we try to find as many candidates as possible
            for queue in self.aggregated.values() {
                if let Some(previous) = queue.previous(candidate_hash) {
                    candidates.push_back(previous);
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use gear_core::ids::ActorId;
    use hypercore_signer::{Address, Signature};

    #[derive(Clone, Debug)]
    pub struct MyComm([u8; 2]);

    impl SeqHash for MyComm {
        fn hash(&self) -> H256 {
            hash(&self.0[..])
        }
    }

    fn signer(id: u8) -> Address {
        let mut array = [0; 20];
        array[0] = id;
        Address(array)
    }

    fn signature(id: u8) -> Signature {
        let mut array = [0; 65];
        array[0] = id;
        Signature::from(array)
    }

    fn block_hash(id: u8) -> H256 {
        let mut array = [0; 32];
        array[0] = id;
        H256::from(array)
    }

    fn pid(id: u8) -> ActorId {
        let mut array = [0; 32];
        array[0] = id;
        ActorId::from(array)
    }

    fn state_id(id: u8) -> H256 {
        let mut array = [0; 32];
        array[0] = id;
        H256::from(array)
    }

    fn gen_commitment(
        signature_id: u8,
        commitments: Vec<(u8, u8)>,
    ) -> AggregatedCommitments<MyComm> {
        let commitments = commitments
            .into_iter()
            .map(|v| MyComm([v.0, v.1]))
            .collect();

        AggregatedCommitments {
            commitments,
            signature: signature(signature_id),
        }
    }

    #[test]
    fn simple() {
        // aggregator with threshold 1
        let mut aggregator = Aggregator::new(1);

        aggregator.push(signer(1), gen_commitment(0, vec![(1, 1)]));

        let root = aggregator
            .find_root()
            .expect("Failed to generate root commitment");

        assert_eq!(root.signatures.len(), 1);
        assert_eq!(root.commitments.len(), 1);

        // aggregator with threshold 1
        let mut aggregator = Aggregator::new(1);

        aggregator.push(signer(1), gen_commitment(0, vec![(1, 1)]));
        aggregator.push(signer(1), gen_commitment(1, vec![(1, 1), (2, 2)]));

        let root = aggregator
            .find_root()
            .expect("Failed to generate root commitment");

        assert_eq!(root.signatures.len(), 1);

        // should be latest commitment
        assert_eq!(root.commitments.len(), 2);
    }
}