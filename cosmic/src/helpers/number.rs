#[allow(dead_code)]
pub fn to_cent(float: f64) -> i64 {
    (float * 100.00).round() as i64
}

#[allow(dead_code)]
pub fn from_cent(int: i64) -> f64 {
    (int as f64) / 100.00
}
