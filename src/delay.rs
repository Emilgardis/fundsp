use super::audionode::*;
use super::math::*;
use super::*;
use numeric_array::typenum::*;

/// Fixed delay.
#[derive(Clone)]
pub struct DelayNode<T: Float> {
    buffer: Vec<T>,
    i: usize,
    delay_time: f64,
}

impl<T: Float> DelayNode<T> {
    pub fn new(delay_time: f64, sample_rate: f64) -> DelayNode<T> {
        let mut ac = DelayNode {
            buffer: vec![],
            i: 0,
            delay_time,
        };
        ac.reset(Some(sample_rate));
        ac
    }
}

impl<T: Float> AudioNode for DelayNode<T> {
    const ID: u64 = 13;
    type Sample = T;
    type Inputs = U1;
    type Outputs = U1;

    #[inline]
    fn reset(&mut self, sample_rate: Option<f64>) {
        if let Some(sample_rate) = sample_rate {
            let buffer_length = ceil(self.delay_time * sample_rate);
            self.buffer
                .resize(max(1, buffer_length as usize), T::zero());
        }
        self.i = 0;
        for x in self.buffer.iter_mut() {
            *x = T::zero();
        }
    }

    #[inline]
    fn tick(
        &mut self,
        input: &Frame<Self::Sample, Self::Inputs>,
    ) -> Frame<Self::Sample, Self::Outputs> {
        let output = self.buffer[self.i];
        self.buffer[self.i] = input[0];
        self.i += 1;
        if self.i >= self.buffer.len() {
            self.i = 0;
        }
        [output].into()
    }
}
