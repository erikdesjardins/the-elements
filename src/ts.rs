pub fn to_tenths_celsius(sense: u16) -> i32 {
    const MAX_SENSE: u64 = 1 << 12; // LSB
    const FULL_SCALE: u64 = 3_200_000; // uV
    const AVG_SLOPE: i32 = 4_300 / 10; // uV / (1/10)C
    const V_25: i32 = 1_430_000; // uV
    const OFFSET_25: i32 = 250; // (1/10)C

    let sense = u64::from(sense);
    let v_sense = ((sense * FULL_SCALE) / MAX_SENSE) as i32; // uV
    ((V_25 - v_sense) / AVG_SLOPE) + OFFSET_25 // (1/10)C
}
