use std::ops::{Index, Range};

pub struct TimeAxisFloat(Range<i64>); //TODO: make this generic afterwards

impl TimeAxisFloat {
    fn from_range(range: Range<i64>) -> Self {
        TimeAxisFloat(range)
    }
}
impl Index<i64> for TimeAxisFloat {
    type Output = f64;

    fn index(&self, index: i64) -> &f64 {
        &0.
    }
}

#[cfg(test)]
mod tests {
    use std::f64::NAN;

    #[test]
    fn create_axis() {
        let axis = crate::TimeAxisFloat(5..10);
        assert_eq!(axis.0.start, 5);
        assert_eq!(axis.0.end, 10);
    }

    #[test]
    fn index_axis_within_range() {
        let axis = crate::TimeAxisFloat(5..10);
        assert_eq!(axis[5], 0f64);
        assert_eq!(axis[6], 0f64);
        assert_eq!(axis[7], 0f64);
        assert_eq!(axis[8], 0f64);
        assert_eq!(axis[9], 0f64);
        assert_eq!(axis[10], 0f64);
    }

    #[test]
    #[should_panic]
    fn index_axis_out_of_range() {
        let axis = crate::TimeAxisFloat(5..10);
        assert_eq!(axis[4], NAN);
    }

}
