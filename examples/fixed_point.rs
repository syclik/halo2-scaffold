#[macro_use]
extern crate assert_float_eq;

#[derive(Debug)]
pub struct Q63_63 {
    data: u128,
}

impl Q63_63 {
    fn new(data: u128) -> Self {
        Q63_63 { data: data }
    }

    fn from_f64(data: f64) -> Self {
        let d = data.abs();
        let unsigned_data = ((d as u128) << 64) as u128
            + ((d % 1.0) * f64::powf(2.0, 63.0)) as u128
            | (data.is_sign_negative() as u128) << 127;

        Q63_63 { data: unsigned_data }
    }

    fn as_f64(&self) -> f64 {
        let unsigned_data = self.data << 1 >> 1;
        let signum = (0.5f64 - ((self.data >> 127) as f64)) * 2.0;

        signum
            * ((unsigned_data >> 64) as f64
                + ((unsigned_data << 64) >> 64) as f64 / f64::powf(2.0, 63.0))
    }

    fn as_str(&self) -> String {
        format!("0b{:128b}", self.data)
    }
}

fn main() {
    let m = 123.456_f64;
    println!("m = {:.5}", m);

    let a = m as i64;
    println!("a = {:.5}", a);

    let b = m % 1.0;
    println!("b = {:.5}", b);
    let b2 = (b * (f64::powf(2.0, 63.0))) as u64;
    println!("b2 = {:}", b2);

    /*let q = Q63_63::new(0);
    println!("Q63_63: {:?}", q);
    println!("Q63_63.as_f64: {}", q.as_f64());

    let q1 = Q63_63::new(1000000_u128);
    println!("Q63_63: {:?}", q1);

    println!("Q63_63.as_f64: {}", q1.as_f64());
    */
    let q2 = Q63_63::from_f64(-1000000.456);
    println!("Q63_63: {:?}", q2);

    println!("Q63_63.as_f64: {:.5}", q2.as_f64());

    let q3 = Q63_63::from_f64(1000000.456);
    println!("Q63_63.as_f64: {}", q3.as_f64());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q63_63_zero() {
        let f = Q63_63::new(0);

        assert_eq!(0_u128, f.data);
    }

    #[test]
    fn q63_63_roundtrip() {
        let examples: [f64; 3] = [12345.6789, 987.654, 123.0];

        for example in examples.iter() {
            let example_neg = -example;

            let f_pos = Q63_63::from_f64(*example);
            assert_float_absolute_eq!(*example, f_pos.as_f64(), 0.01);

            let f_neg = Q63_63::from_f64(example_neg);
            assert_float_absolute_eq!(example_neg, f_neg.as_f64(), 0.01);

            assert_ne!(f_pos.data, f_neg.data);
            assert_eq!(f_pos.data, f_neg.data << 1 >> 1);
        }
    }
}
