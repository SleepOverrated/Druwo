use core::{f64, panic};
use std::{f32, usize};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f32),
    Double(f64),
    Str(String),
}

// There probably is a better way to do this but it works
impl Value {

    fn to_bytecode(&self) -> Vec<u8> {
        match self {
            Value::Int(i) => {
                let mut bytes = vec![0]; // Data type tag for Integer
                bytes.extend_from_slice(&i.to_le_bytes());
                bytes
            },
            Value::Float(f) => {
                let mut bytes = vec![1]; // Data type tag for Float
                bytes.extend_from_slice(&f.to_le_bytes());
                bytes
            },
            Value::Double(d) => {
                let mut bytes = vec![2]; // Data type tag for Double
                bytes.extend_from_slice(&d.to_le_bytes());
                bytes
            },
            Value::Str(s) => {
                let mut bytes = vec![3]; // Data type tag for String
                let s_bytes = s.as_bytes();
                bytes.extend_from_slice(&(s_bytes.len()).to_le_bytes());
                bytes.extend_from_slice(s_bytes);
                bytes
            }
        }

    }


    pub fn add(&self, other: &Self) -> Self {
        use Value::*;
        match (self, other) {
            (Int(x), Int(y)) => Int(x + y),
            (Float(x), Float(y)) => Float(x + y),
            (Double(x), Double(y)) => Double(x + y),

            (Int(x), Float(y)) | (Float(y), Int(x)) => Float(*x as f32 + y),
            (Int(x), Double(y)) | (Double(y), Int(x)) => Double(*x as f64 + y),
            (Float(x), Double(y)) | (Double(y), Float(x)) => Double(*x as f64 + y),

           
            (Str(s), Int(i)) => Str(format!("{}{}", s, i)),
            (Int(i), Str(s)) => Str(format!("{}{}", i, s)),
            (Str(s), Float(f)) => Str(format!("{}{}", s, f)),
            (Float(f), Str(s)) => Str(format!("{}{}", f, s)),
            (Str(s), Double(d)) => Str(format!("{}{}", s, d)),
            (Double(d), Str(s)) => Str(format!("{}{}", d, s)),
            (Str(s1), Str(s2)) => Str(format!("{}{}", s1, s2)),
        }
    }

    pub fn sub(&self, other: &Self) -> Self {
        use Value::*;
        match (self, other) {
            (Int(x), Int(y)) => Int(x - y),
            (Float(x), Float(y)) => Float(x - y),
            (Double(x), Double(y)) => Double(x - y),

            (Int(x), Float(y)) => Float(*x as f32 - y),
            (Float(y), Int(x)) => Float(y - *x as f32),
            (Int(x), Double(y)) => Double(*x as f64 - y),
            (Double(y), Int(x)) => Double(y - *x as f64),
            (Float(x), Double(y)) => Double(*x as f64 - y),
            (Double(y), Float(x)) => Double(y - *x as f64),

            (Str(x), Int(y)) => Str(x.split_at(x.len() - *y as usize).0.to_string()),

            _ => panic!("Subtract operation not supported for the given types"),
        }
    }

    pub fn mul(&self, other: &Self) -> Self {
        use Value::*;
        match (self, other) {
            (Int(x), Int(y)) => Int(x * y),
            (Float(x), Float(y)) => Float(x * y),
            (Double(x), Double(y)) => Double(x * y),

            (Int(x), Float(y)) | (Float(y), Int(x)) => Float(*x as f32 * y),
            (Int(x), Double(y)) | (Double(y), Int(x)) => Double(*x as f64 * y),
            (Float(x), Double(y)) | (Double(y), Float(x)) => Double(*x as f64 * y),

            _ => panic!("Multiply operation not supported for the given types"),
        }
    }

    pub fn div(&self, other: &Self) -> Self {
        use Value::*;
        match (self, other) {
            (Int(x), Int(y)) => Float(*x as f32/ *y as f32),
            (Float(x), Float(y)) => Float(x / y),
            (Double(x), Double(y)) => Double(x / y),

            (Int(x), Float(y)) => Float(*x as f32 / y),
            (Float(y), Int(x)) => Float(y / *x as f32),
            (Int(x), Double(y)) => Double(*x as f64 / y),
            (Double(y), Int(x)) => Double(y / *x as f64),
            (Float(x), Double(y)) => Double(*x as f64 / y),
            (Double(y), Float(x)) => Double(y / *x as f64),

            _ => panic!("Divide operation not supported for the given types"),
        }
    }


    /* Return true if self is greater than other*/
    pub fn greater(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Int(x), Int(y)) => x > y,
            (Float(x), Float(y)) => x > y,
            (Double(x), Double(y)) => x > y,


            (Int(x), Float(y)) => *x as f32 > *y,
            (Float(y), Int(x)) => *y > *x as f32,
            (Int(x), Double(y)) => *x as f64 > *y,
            (Double(y), Int(x)) => *y > *x as f64,
            (Float(x), Double(y)) => *x as f64 > *y,
            (Double(y), Float(x)) => *y > *x as f64,


            _ => panic!("Check if greater operation not supported for the given types")
            
        }
    }

    pub fn greater_equal(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Int(x), Int(y)) => x >= y,
            (Float(x), Float(y)) => x >= y,
            (Double(x), Double(y)) => x >= y,


            (Int(x), Float(y)) => *x as f32 >= *y,
            (Float(y), Int(x)) => *y >= *x as f32,
            (Int(x), Double(y)) => *x as f64 >= *y,
            (Double(y), Int(x)) => *y >= *x as f64,
            (Float(x), Double(y)) => *x as f64 >= *y,
            (Double(y), Float(x)) => *y >= *x as f64,


            _ => panic!("Check if greater operation not supported for the given types")
            
        }
    }

    pub fn less(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Int(x), Int(y)) => x < y,
            (Float(x), Float(y)) => x < y,
            (Double(x), Double(y)) => x < y,


            (Int(x), Float(y)) => (*x as f32) < *y,
            (Float(y), Int(x)) => *y < *x as f32,
            (Int(x), Double(y)) => (*x as f64) < *y,
            (Double(y), Int(x)) => *y < *x as f64,
            (Float(x), Double(y)) => (*x as f64) < *y,
            (Double(y), Float(x)) => *y < *x as f64,


            _ => panic!("Check if greater operation not supported for the given types")
            
        }
    }

    pub fn less_equal(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Int(x), Int(y)) => x <= y,
            (Float(x), Float(y)) => x < y,
            (Double(x), Double(y)) => x < y,


            (Int(x), Float(y)) => (*x as f32) <= *y,
            (Float(y), Int(x)) => *y <= *x as f32,
            (Int(x), Double(y)) => (*x as f64) <= *y,
            (Double(y), Int(x)) => *y <= *x as f64,
            (Float(x), Double(y)) => (*x as f64) <= *y,
            (Double(y), Float(x)) => *y <= *x as f64,


            _ => panic!("Check if greater operation not supported for the given types")
            
        }
    }

}
