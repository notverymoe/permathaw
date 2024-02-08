// Copyright 2024 Natalie Baker // AGPLv3 //

pub struct TrackQueue(u64);

impl TrackQueue {

    pub fn advance(&mut self) {
        self.0 = accumulate_zeros_to_right(0);
    }

}

// ////////////////// //
// // Impl methods // //
// ////////////////// //

pub const U64_MSB: u64 = 0x8000_0000_0000_0000;

/// This function shifts right by 1, accumulating 0s at the
/// end. If the input is non-zero, then 1s will be shifted 
/// in at the start. If you understand what this does, then 
/// you need it, if you don't then you don't.
const fn accumulate_zeros_to_right(v: u64) -> u64 {
    // So, let's be honest. This is a weird bit hack, let's break it down.
    //
    // This function attempts to move 0s to the right, collecting 0s at the
    // end and inserting 1s at the start if any zeros moved. You can think
    // about this as 1s representing empty space and 0s representing a box
    // on a converyor. If a box reaches the end, it takes up space and the 
    // next box will push up against it, and the next, until all the boxes
    // are on the right.
    // 
    // It's somewhat confusing that 0s are occupied, however, trust me, the
    // instructions just kinda work out better with 0s as being occupied.
    //
    // Anyway. Breakdown. 
    //
    // BLSR is an instruction from the BMI1 instruction set, what it does is 
    // reset the lowest set bit. That is, it turns the right-most air into a 
    // box. Then we shift right. Then we insert air at the start if the conveyor 
    // isn't full.
    //
    // BLSR then SHR is a little weird. The way to think about it is like this.
    // - The BLSR'd air will be to the left of the end of the conveyor, or a box.
    // - This means that this is the air that would be squished out by a box.
    // - We then shift to the right, this advances the conveyor.
    // - By converting that air to a box then shifting, we increase the number of 
    //      boxes at the end by 1 and then shift one box off the end, maintaining 
    //      the number of boxes.
    //
    // # Worked Examples
    // 
    // Interlaced:
    // Input:  B-B-| (0b0101)
    // BLSR:   B-BB| (0b0100)
    // SHR:    BB-B| (0b0010)
    // Output: -B-B| (0b1010)
    //
    // Interlaced Reverse:
    // Input:  -B-B| (0b1010)
    // BLSR:   -BBB| (0b1000)
    // SHR:    --BB| (0b1100)
    // Output: --BB| (0b1100)
    //
    // Two airs in middle:
    // Input:  B--B| (0b0110)
    // BLSR:   B-BB| (0b0100)
    // SHR:    BB-B| (0b0010)
    // Output: -B-B| (0b1010)
    //
    // Two boxes in middle:
    // Input:  -BB-| (0b1001)
    // BLSR:   -BBB| (0b1000)
    // SHR:    B-BB| (0b0100)
    // Output: --BB| (0b0011)
    //
    // Two airs at end:
    // Input:  BB--| (0b0011)
    // BLSR:   BB-B| (0b0010)
    // SHR:    BBB-| (0b0001)
    // Output: -BB-| (0b1001)
    //
    // Two boxes at end:
    // Input:  --BB| (0b1100)
    // BLSR:   -BBB| (0b1000)
    // SHR:    B-BB| (0b0100)
    // Output: --BB| (0b1100)
    //
    // All boxes case:
    // Input:  BBBB| (0b0000)
    // BLSR:   BBBB| (0b0000)
    // SHR:    BBBB| (0b0000)
    // Output: BBBB| (0b0000)
    //
    // No boxes case:
    // Input:  ----| (0b1111)
    // BLSR:   ---B| (0b1110)
    // SHR:    B---| (0b0111)
    // Output: ----| (0b1111)
    //
    let blsr = v & (v.wrapping_sub(1));
    let shr  = blsr >> 1;
    let msb  = if v == 0 { U64_MSB } else { 0 };
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
            let result = accumulate_zeros_to_right(value);
            assert!(result == expected, "Case {i} [{value:#06b}, {expected:#06b}]: Got {result:#06b}");
        }

    }
}