// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::Component;

use super::util::accumulate_zeros_to_right;

#[derive(Clone, Copy, Component)]
pub struct TrackQueue(u64);

impl TrackQueue {

    #[must_use]
    pub const fn next(&self) -> Self {
        Self(accumulate_zeros_to_right(self.0))
    }

    pub fn advance(&mut self) {
        *self = self.next();
    }

    #[must_use]
    pub const fn get(&self, idx: usize) -> bool {
        (self.0 >> idx) & 1 == 0   
    }

    pub fn remove(&mut self, idx: usize) {
        self.0 |= 1 << idx;
    }

    pub fn insert(&mut self, idx: usize) {
        self.0 &= !(1 << idx);
    }

}

impl Default for TrackQueue {
    fn default() -> Self {
        Self(u64::MAX)
    }
}

impl core::fmt::Debug for TrackQueue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TrackQueue").field(&format!("{:#064b}", self.0)).finish()
    }
}

struct TrackQueueIter<'a> {
    queue: &'a TrackQueue,
    idx:   usize,
}

impl<'a> Iterator for TrackQueueIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.idx >= 64 { return None; }
            let idx = self.idx;
            self.idx += 1;
            if self.queue.get(idx) {
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
    use crate::track::util::U64_MSB;

    #[test]
    pub fn test_queue() {
        let mut queue = TrackQueue::default();
        assert_eq!(queue.0, u64::MAX);

        // Insert at front
        queue.insert(0);
        assert_eq!(queue.0, !1);
        queue.remove(0);
        assert_eq!(queue.0, u64::MAX);

        // Insert at back
        queue.insert(63);
        assert_eq!(queue.0, !U64_MSB);
        queue.remove(63);
        assert_eq!(queue.0, u64::MAX);

        // Check advance blocked 1
        queue.insert(0);
        queue.advance();
        assert_eq!(queue.0, !1);
        queue.remove(0);
        assert_eq!(queue.0, u64::MAX);

        // Check advance blocked 2
        queue.insert(0);
        queue.insert(1);
        queue.advance();
        assert_eq!(queue.0, !3);
        queue.remove(0);
        assert_eq!(queue.0, !2);

        // Check advance unblocked 1
        queue.insert(63);
        queue.advance();
        assert_eq!(queue.0, !(U64_MSB >> 1));
        queue.remove(62);
        assert_eq!(queue.0, u64::MAX);

        // Check advance unblocked 2
        queue.insert(63);
        queue.insert(62);
        queue.advance();
        assert_eq!(queue.0, !(U64_MSB >> 1));
        queue.remove(62);
        assert_eq!(queue.0, u64::MAX);
    }
}