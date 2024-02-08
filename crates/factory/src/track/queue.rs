// Copyright 2024 Natalie Baker // AGPLv3 //

pub struct TrackQueue(u64);

impl TrackQueue {

    pub fn advance(&mut self) {
        self.0 = advance_queue(0);
    }

}

// ////////////////// //
// // Impl methods // //
// ////////////////// //

pub const U64_MSB: u64 = 0x8000_0000_0000_0000;

const fn advance_queue(v: u64) -> u64 {
    let blsr = v & (v.wrapping_sub(1));
    let shr  = blsr >> 1;
    let msb  = U64_MSB * (v != 0) as u64;
    msb | shr
}

// /////////// //
// // Tests // //
// /////////// //

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_shifting() {

        let test_set = [
            [0b0000,           0b0000],
            [0b0001, U64_MSB         ],
            [0b0010, U64_MSB         ],
            [0b0100, U64_MSB         ],
            [0b1000, U64_MSB         ],
            [0b0011, U64_MSB | 0b0001],
            [0b0110, U64_MSB | 0b0010],
            [0b1100, U64_MSB | 0b0100],
            [0b0101, U64_MSB | 0b0010],
            [0b1001, U64_MSB | 0b0100],
            [0b1101, U64_MSB | 0b0110],
            [u64::MAX, u64::MAX],
        ];

        for (i, [value, expected]) in test_set.into_iter().enumerate() {
            let result = advance_queue(value);
            assert!(result == expected, "Case {i} [{value:#06b}, {expected:#06b}]: Got {result:#06b}");
        }

    }
}