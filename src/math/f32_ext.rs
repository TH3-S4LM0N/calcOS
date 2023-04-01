pub use {
    micromath::F32Ext as mmF32Ext,
    self::F32Ext as cF32Ext
};

pub trait F32Ext {
    /// Returns true if the value is even.
    fn is_even(&self) -> bool;
    fn c_powf(&self, power: f32) -> f32;
}

impl F32Ext for f32 {
    fn is_even(&self) -> bool {
        if &self.trunc() != self {
            unreachable!()
        }
        let dig = self % 10.0;

        dig == 0.0 || dig == 2.0 || dig == 4.0 || dig == 6.0 || dig == 8.0
    }

    fn c_powf(&self, power: f32) -> f32 {
        let mut answer: f32 = 0.0;
        for _ in 0..=power as i32 {
            answer = self * self;
        }
        return answer;
    }
}