use num;

pub trait AverageTrait<ValueT>
where
    ValueT: num::Float,
{
    fn count(&self) -> usize;

    fn value(&self) -> ValueT;

    fn add(&mut self, value: ValueT);

    fn clear(&mut self);
}
