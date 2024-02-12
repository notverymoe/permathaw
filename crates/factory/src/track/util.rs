// Copyright 2024 Natalie Baker // AGPLv3 //

pub const U64_MSB: u64 = 0x8000_0000_0000_0000;

/// Shift right by 1, without shifting 0s off the end thus accumulating them 
/// instead. Shifts in a 1 at the MSB if the value was non-zero, otherwise a 0.
/// This is used to simulate a conveyor belt.
pub const fn accumulate_zeros_to_right(v: u64) -> u64 {
    // So, let's be honest. This is a weird bit hack, let's break it down.
    //
    // This function attempts to move 0s to the right, collecting 0s at the end 
    // and inserting 1s at the start if any zeros moved. You can think about 
    // this as 1s representing empty space and 0s representing a box on a 
    // converyor. If a box reaches the end, it takes up space and the next box 
    // will push up against it, and the next, until all the boxes are on the 
    // right. It's somewhat confusing that 0s are occupied, however, trust me, 
    // the instructions just kinda work out better with 0s as being occupied.
    //
    // BLSR is an asm instruction from BMI1, it simply resets the lowest set 
    // bit. In our terms, this turns the frontmost air into a box. We then shift
    // right and insert air at the start if the conveyor isn't full. BLSR then 
    // SHR is a little weird. The way to think about it is like this:
    // - The BLSR'd air will be to the left of the end of the conveyor, or a box
    // - This means that this is the air that would be squished out by a box.
    // - We then shift to the right, this advances the conveyor.
    // - By converting that air to a box then shifting, we increase the number 
    //      of boxes at the end by 1 and then shift one box off the end, 
    //      maintaining the number of boxes.
    //
    // # Worked Examples
    //-------------------------------------------------------------------------
    // Interlaced:           || Interlaced Reverse:   || Two airs in middle:
    // Input:  B-B- (0b0101) || Input:  -B-B (0b1010) || Input:  B--B (0b0110)
    // BLSR:   B-BB (0b0100) || BLSR:   -BBB (0b1000) || BLSR:   B-BB (0b0100)
    // SHR:    BB-B (0b0010) || SHR:    --BB (0b1100) || SHR:    BB-B (0b0010)
    // Output: -B-B (0b1010) || Output: --BB (0b1100) || Output: -B-B (0b1010)
    //-------------------------------------------------------------------------
    // Two airs at end:      || Two boxes at end:     || Two boxes in middle:
    // Input:  BB-- (0b0011) || Input:  --BB (0b1100) || Input:  -BB- (0b1001)
    // BLSR:   BB-B (0b0010) || BLSR:   -BBB (0b1000) || BLSR:   -BBB (0b1000)
    // SHR:    BBB- (0b0001) || SHR:    B-BB (0b0100) || SHR:    B-BB (0b0100)
    // Output: -BB- (0b1001) || Output: --BB (0b1100) || Output: --BB (0b0011)
    //-------------------------------------------------------------------------
    // All boxes case:       || All air case:         ||
    // Input:  BBBB (0b0000) || Input:  ---- (0b1111) ||
    // BLSR:   BBBB (0b0000) || BLSR:   ---B (0b1110) ||
    // SHR:    BBBB (0b0000) || SHR:    B--- (0b0111) ||
    // Output: BBBB (0b0000) || Output: ---- (0b1111) ||
    //---------------------------------------------------------------------------
    let blsr = v & (v.wrapping_sub(1));
    let shr  = blsr >> 1;
    let msb  = if v == 0 { 0 } else { U64_MSB }; // OPT this is faster in debug (no panic), and gets optimized terse.
    msb | shr
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_accumulate_zeros_to_right() {

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