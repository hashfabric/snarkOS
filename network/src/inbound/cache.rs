// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkOS library.

// The snarkOS library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkOS library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkOS library. If not, see <https://www.gnu.org/licenses/>.

use crate::Payload;

use snarkvm_dpc::block::BlockHeader;

use circular_queue::CircularQueue;
use twox_hash::xxh3::hash64;

#[derive(Debug, Clone)]
pub struct Cache<const N: usize> {
    queue: CircularQueue<u64>,
}

impl<const N: usize> Default for Cache<N> {
    fn default() -> Self {
        Self {
            queue: CircularQueue::with_capacity(N),
        }
    }
}

impl<const N: usize> Cache<N> {
    pub fn contains(&self, payload: &Payload) -> bool {
        let hash = if let Payload::Block(bytes, _) = payload {
            hash64(&bytes[..BlockHeader::size()])
        } else {
            unreachable!("Only blocks are cached for now");
        };

        self.queue.iter().any(|&e| e == hash)
    }

    pub fn push(&mut self, payload: &Payload) {
        let hash = if let Payload::Block(bytes, _) = payload {
            hash64(&bytes[..BlockHeader::size()])
        } else {
            unreachable!("Only blocks are cached for now");
        };

        self.queue.push(hash);
    }
}
