const SIMPLE8B_BITS: [usize; 16] = [0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 12, 15, 20, 30, 60];
const SIMPLE8B_N: [usize; 16] = [240, 120, 60, 30, 20, 15, 12, 10, 8, 7, 6, 5, 4, 3, 2, 1];

fn get_min_selector(i: u64) -> Result<usize, &'static str> {
    let bits = 64 - i.leading_zeros();
    if bits == 0 {
        return Ok(0);
    } else if bits <= 8 {
        return Ok((bits + 1) as usize);
    } else if bits <= 10 {
        return Ok(10);
    } else if bits <= 12 {
        return Ok(11);
    } else if bits <= 15 {
        return Ok(12);
    } else if bits <= 20 {
        return Ok(13);
    } else if bits <= 30 {
        return Ok(14);
    } else if bits <= 60 {
        return Ok(15);
    }
    Err("value too large")
}

/// Pack as many values from data into a single u64
/// All values of data must be smaller than 2^60
/// Returns the number of values in result or an error if a value is too large
pub fn pack(data: &[u64], result: &mut u64) -> Result<usize, &'static str> {
    let mut selector = 0;
    let mut count: usize = 0;
    // add more values to pool and keep track of selector until the maximum number of bits is exceeded
    for &v in data.iter() {
        let selector_next = usize::max(get_min_selector(v)?, selector);
        if count >= SIMPLE8B_N[selector_next] {
            break;
        }
        selector = selector_next;
        count += 1;
    }
    // if not all values fit, make sure to pack at most <count> values
    if count != data.len() {
        while SIMPLE8B_N[selector] > count {
            selector += 1;
        }
    }
    let mut packed = 0;
    for &v in data.iter().take(SIMPLE8B_N[selector]).rev() {
        packed <<= SIMPLE8B_BITS[selector];
        packed |= v;
    }
    *result = packed | (selector as u64) << 60;
    Ok(count)
}

/// Count the number of values packed inside the u64
pub fn count_packed(v: u64) -> usize {
    SIMPLE8B_N[(v >> 60) as usize]
}

/// Decode values from a packed u64 into output
pub fn unpack(v: u64, output: &mut [u64]) -> usize {
    let count = usize::min(count_packed(v), output.len());
    let bits = SIMPLE8B_BITS[(v >> 60) as usize];
    let mask = u64::max_value() >> (64 - bits);
    let mut v = v;
    for i in 0..count {
        output[i] = v & mask;
        v >>= bits;
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_too_large() {
        let values = [2, 76, 3, (u64::max_value() >> 4) + 1, 7, 2];
        let mut r = 0;
        let res = pack(&values, &mut r);
        match res {
            Ok(_) => panic!("no error"),
            Err(_) => (),
        }
    }
}
