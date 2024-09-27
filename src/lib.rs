
pub fn invsqrt(x: f32) -> f32 {
    unsafe {
        // y = 1/sqrt(x) = 2 ^ (-1/2 * log2(x))
        let dx: i32 = *(&x as *const f32 as *const i32);
        let dy: i32 = 0x5F3759DF - (dx >> 1);  // Magic number
        let mut y: f32 = *(&dy as *const i32 as *const f32);

        // Newton's method
        let threehalfs = 1.5f32;
        let x2 = x * 0.5f32;
        y = y * (threehalfs - (x2 * y * y));  // 1st iteration
        y = y * (threehalfs - (x2 * y * y));  // 2nd iteration

        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fabs(x: f32) -> f32 {
        unsafe {
            let dx: i32 = *(&x as *const f32 as *const i32);
            let dy: i32 = 0x7FFFFFFF & dx;  // flip first bit (sign bit)
            let y: f32 = *(&dy as *const i32 as *const f32);

            y
        }
    }

    #[test]
    fn test_invsqrt() {
        const N: usize = 100usize;

        let mut ave_error = 0f64;
        let mut max_error = 0f64;

        (1..(N+1)).for_each(|x| {
            // use sqrt()
            let y = 1.0f32 / (x as f32).sqrt();

            // fast inverse square root
            let fx = x as f32;
            let fy = invsqrt(fx);

            // error
            let error = fabs((fy - y) / y) as f64 * 100f64;
            ave_error += error;
            max_error = {
                if error > max_error { error }
                else { max_error }
            };
        });
        ave_error /= 100f64;

        assert!(ave_error < 0.001f64);
        assert!(max_error < 0.001f64);
    }
}
