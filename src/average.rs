use num;

use super::average_trait::AverageTrait;

#[derive(Default)]
pub struct Average<ValueT>
where
    ValueT: num::Float,
{
    count: usize,
    sum: ValueT,
}

impl<ValueT> Average<ValueT>
where
    ValueT: num::Float,
{
    pub fn new() -> Self {
        Average {
            count: 0,
            sum: ValueT::zero(),
        }
    }
}

impl<ValueT> AverageTrait<ValueT> for Average<ValueT>
where
    ValueT: num::Float,
{
    fn count(&self) -> usize {
        self.count
    }

    fn value(&self) -> ValueT {
        self.sum / ValueT::from(self.count).unwrap()
    }

    fn add(&mut self, value: ValueT) {
        self.count += 1;
        self.sum = self.sum + value;
    }

    fn clear(&mut self) {
        self.count = 0;
        self.sum = ValueT::zero();
    }
}

#[cfg(test)]
mod test {

    use super::Average;
    use crate::average::AverageTrait;

    #[test]
    fn test() {
        let mut a: Average<f64> = Average::new();

        assert!(a.count() == 0);
        assert!(a.value().is_nan());

        a.add(0.0);
        a.add(3.0);
        a.add(2.0);

        assert!(a.count() == 3);
        assert!((a.value() - 5.0 / 3.0).abs() <= 1e-10);

        a.clear();

        assert!(a.count() == 0);
        assert!(a.value().is_nan());

        a.add(1.0);
        a.add(4.0);

        assert!(a.count() == 2);
        assert!((a.value() - 5.0 / 2.0).abs() <= 1e-10);
    }
}
