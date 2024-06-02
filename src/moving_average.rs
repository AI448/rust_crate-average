use num;

use super::average_trait::AverageTrait;

pub struct MovingAverage<ValueT>
where
    ValueT: num::Float,
{
    ring_buffer: Vec<ValueT>,
    first: usize,
    count: usize,
    sum: ValueT,
}

impl<ValueT> MovingAverage<ValueT>
where
    ValueT: num::Float,
{
    pub fn new(time_constant: usize) -> Self {
        MovingAverage {
            ring_buffer: Vec::from_iter(std::iter::repeat(ValueT::nan()).take(time_constant)),
            first: 0,
            count: 0,
            sum: ValueT::zero(),
        }
    }

    pub fn time_constant(&self) -> usize {
        self.ring_buffer.len()
    }
}

impl<ValueT> AverageTrait<ValueT> for MovingAverage<ValueT>
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
        if self.count == self.ring_buffer.len() {
            self.sum = self.sum - self.ring_buffer[self.first];
            self.first = (self.first + 1) % self.ring_buffer.len();
        } else {
            self.count += 1;
        }
        let last = (self.first + self.count - 1) % self.ring_buffer.len();
        self.ring_buffer[last] = value;
        self.sum = self.sum + value;
    }

    fn clear(&mut self) {
        self.first = 0;
        self.count = 0;
        self.sum = ValueT::zero();
    }
}

#[cfg(test)]
mod test {

    use super::AverageTrait;
    use super::MovingAverage;

    #[test]
    fn test() {
        let mut a: MovingAverage<f64> = MovingAverage::new(3);

        assert!(a.time_constant() == 3);
        assert!(a.count() == 0);
        assert!(a.value().is_nan());

        a.add(2f64);
        a.add(3f64);

        assert!(a.count() == 2);
        assert!((a.value() - 5.0 / 2.0).abs() <= 1e-10);

        a.clear();

        assert!(a.count() == 0);
        assert!(a.value().is_nan());

        a.add(1f64);
        a.add(2f64);
        a.add(3f64);
        a.add(4f64);

        assert!(a.count() == 3);
        assert!((a.value() - 9.0 / 3.0).abs() <= 1e-10);
    }
}
