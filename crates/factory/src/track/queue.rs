// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::Component;

use super::util::accumulate_zeros_to_right;

#[derive(Clone, Copy, Component, PartialEq, Eq)]
pub struct TrackQueue(u64);

impl TrackQueue {

    #[must_use]
    pub const fn next(self) -> Self {
        Self(accumulate_zeros_to_right(self.0))
    }

    #[must_use]
    pub const fn has(self, idx: usize) -> bool {
        (self.0 >> idx) & 1 == 0   
    }

    #[must_use]
    pub const fn without(self, idx: usize) -> Self {
        Self(self.0 | (1 << idx))
    }

    #[must_use]
    pub const fn with(self, idx: usize) -> Self {
        Self(self.0 & !(1 << idx))
    }

}

impl Default for TrackQueue {
    fn default() -> Self {
        Self(u64::MAX)
    }
}

impl core::fmt::Debug for TrackQueue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TrackQueue").field(&format!("{:#066b}", self.0)).finish()
    }
}

struct TrackQueueIter<'a> {
    queue: &'a TrackQueue,
    idx:   usize,
}

impl<'a> Iterator for TrackQueueIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        // TODO OPT
        loop {
            if self.idx >= 64 { return None; }
            let idx = self.idx;
            self.idx += 1;
            if self.queue.has(idx) {
                return Some(idx);
            }
        }
    }
}

// ////////////////// //
// // Impl methods // //
// ////////////////// //


// /////////// //
// // Tests // //
// /////////// //

#[cfg(test)]
mod test {
    use super::*;

    const fn conveyor_mask<const N: usize>(pattern: [usize; N]) -> TrackQueue {
        let mut result = 0;
        let mut index  = 0;
        loop {
            if index >= N { break; }
            result |= 1 << pattern[index];
            index += 1;
        }
        TrackQueue(!result)
    }

    #[test]
    pub fn test_queue() {
        let mut queue = TrackQueue::default();
        assert_eq!(queue, conveyor_mask([]));

        // Insert at front
        queue = queue.with(0);
        assert_eq!(queue, conveyor_mask([0]));
        queue = queue.without(0);
        assert_eq!(queue, conveyor_mask([]));

        // Insert at back
        queue = queue.with(63);
        assert_eq!(queue, conveyor_mask([63]));
        queue = queue.without(63);
        assert_eq!(queue, conveyor_mask([]));

        // Check advance blocked 1
        queue = queue.with(0);
        queue = queue.next();
        assert_eq!(queue, conveyor_mask([0]));
        queue = queue.without(0);
        assert_eq!(queue, conveyor_mask([]));

        // Check advance blocked 2
        queue = queue.with(0);
        queue = queue.with(1);
        queue = queue.next();
        assert_eq!(queue, conveyor_mask([1, 0]));
        queue = queue.without(0);
        assert_eq!(queue, conveyor_mask([1]));
        queue = queue.next();
        assert_eq!(queue, conveyor_mask([0]));
        queue = queue.without(0);
        assert_eq!(queue, conveyor_mask([]));

        // Check advance unblocked 1
        queue = queue.with(63);
        queue = queue.next();
        assert_eq!(queue, conveyor_mask([62]));
        queue = queue.without(62);
        assert_eq!(queue, conveyor_mask([]));

        // Check advance unblocked 2
        queue = queue.with(63);
        queue = queue.with(62);
        queue = queue.next();
        assert_eq!(queue, conveyor_mask([62, 61]));
        queue = queue.without(61);
        assert_eq!(queue, conveyor_mask([62]));
        queue = queue.next();
        assert_eq!(queue, conveyor_mask([61]));
        queue = queue.without(61);
        assert_eq!(queue, conveyor_mask([]));
    }
}