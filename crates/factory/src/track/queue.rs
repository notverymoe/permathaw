// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use super::util::accumulate_zeros_to_right_nonzero_unchecked;

#[derive(Clone, Copy, PartialEq, Eq, Component)]
pub struct TrackQueue(u64);

impl TrackQueue {

    #[must_use]
    pub const fn next(self) -> Self {
        Self(accumulate_zeros_to_right_nonzero_unchecked(self.0))
    }

    #[must_use]
    pub const fn has(self, idx: usize) -> bool {
        (self.0 >> idx) & 1 == 0   
    }

    #[must_use]
    pub const fn has_many(self, mask: u64) -> bool {
        (!self.0 & !mask) == !mask
    }

    #[must_use]
    pub const fn without(self, idx: usize) -> Self {
        Self(self.0 | (1 << idx))
    }

    #[must_use]
    pub const fn with(self, idx: usize) -> Self {
        Self(self.0 & !(1 << idx))
    }

    #[must_use]
    pub const fn to_raw(self) -> u64 {
        self.0
    }

    #[must_use]
    pub const fn from_occupancy_list<const N: usize>(pattern: [usize; N]) -> Self {
        let mut result = 0;
        let mut index  = 0;
        loop {
            if index >= N { break; }
            result |= 1 << pattern[index];
            index += 1;
        }
        Self(!result)
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

    #[test]
    pub fn test_queue() {
        let mut queue = TrackQueue::default();
        assert_eq!(queue, TrackQueue::from_occupancy_list([]));

        // Insert at front
        queue = queue.with(0);
        assert_eq!(queue, TrackQueue::from_occupancy_list([0]));
        queue = queue.without(0);
        assert_eq!(queue, TrackQueue::from_occupancy_list([]));

        // Insert at back
        queue = queue.with(63);
        assert_eq!(queue, TrackQueue::from_occupancy_list([63]));
        queue = queue.without(63);
        assert_eq!(queue, TrackQueue::from_occupancy_list([]));

        // Check advance blocked 1
        queue = queue.with(0);
        queue = queue.next();
        assert_eq!(queue, TrackQueue::from_occupancy_list([0]));
        queue = queue.without(0);
        assert_eq!(queue, TrackQueue::from_occupancy_list([]));

        // Check advance blocked 2
        queue = queue.with(0);
        queue = queue.with(1);
        queue = queue.next();
        assert_eq!(queue, TrackQueue::from_occupancy_list([1, 0]));
        queue = queue.without(0);
        assert_eq!(queue, TrackQueue::from_occupancy_list([1]));
        queue = queue.next();
        assert_eq!(queue, TrackQueue::from_occupancy_list([0]));
        queue = queue.without(0);
        assert_eq!(queue, TrackQueue::from_occupancy_list([]));

        // Check advance unblocked 1
        queue = queue.with(63);
        queue = queue.next();
        assert_eq!(queue, TrackQueue::from_occupancy_list([62]));
        queue = queue.without(62);
        assert_eq!(queue, TrackQueue::from_occupancy_list([]));

        // Check advance unblocked 2
        queue = queue.with(63);
        queue = queue.with(62);
        queue = queue.next();
        assert_eq!(queue, TrackQueue::from_occupancy_list([62, 61]));
        queue = queue.without(61);
        assert_eq!(queue, TrackQueue::from_occupancy_list([62]));
        queue = queue.next();
        assert_eq!(queue, TrackQueue::from_occupancy_list([61]));
        queue = queue.without(61);
        assert_eq!(queue, TrackQueue::from_occupancy_list([]));
    }
}