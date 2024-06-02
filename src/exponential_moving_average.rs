use num;

use super::average_trait::AverageTrait;

pub struct ExponentialMovingAverage<ValueT>
where
    ValueT: num::Float,
{
    time_constant: usize,
    count: usize,
    mean: ValueT,
}

impl<ValueT> ExponentialMovingAverage<ValueT>
where
    ValueT: num::Float,
{
    pub fn new(time_constant: usize) -> Self {
        assert!(time_constant > 0);
        ExponentialMovingAverage {
            time_constant: time_constant,
            count: 0,
            mean: ValueT::zero(),
        }
    }

    pub fn time_constant(&self) -> usize {
        self.time_constant
    }
}

impl<ValueT> AverageTrait<ValueT> for ExponentialMovingAverage<ValueT>
where
    ValueT: num::Float,
{
    fn count(&self) -> usize {
        self.count
    }

    fn value(&self) -> ValueT {
        if self.count != 0 {
            self.mean
        } else {
            ValueT::nan()
        }
    }

    fn add(&mut self, value: ValueT) {
        self.count += 1;
        let t = ValueT::from(self.time_constant.min(self.count)).unwrap();
        self.mean = ((t - ValueT::one()) * self.mean + value) / t;
    }

    fn clear(&mut self) {
        self.count = 0;
        self.mean = ValueT::zero();
    }
}

#[cfg(test)]
mod test {

    use super::AverageTrait;
    use super::ExponentialMovingAverage;

    #[test]
    fn test() {
        let mut a: ExponentialMovingAverage<f64> = ExponentialMovingAverage::new(3);

        assert!(a.count() == 0);
        assert!(a.value().is_nan());

        a.add(2.0);
        a.add(3.0);

        assert!(a.count() == 2);
        assert!((a.value() - 5.0 / 2.0).abs() <= 1e-10);

        a.clear();

        assert!(a.count() == 0);
        assert!(a.value().is_nan());

        a.add(1.0);
        a.add(2.0);
        a.add(3.0);
        a.add(4.0);

        assert!(a.count() == 4);
        assert!(a.value() > 4.0 / 4.0);
        assert!(a.value() < 16.0 / 4.0);
    }
}
