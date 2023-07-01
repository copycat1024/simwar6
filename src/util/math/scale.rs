#[derive(Default)]
pub struct Scale {
    max: i32,
    value: i32,
    scale: i32,
    scale_value: i32,
}

impl Scale {
    pub fn set_value_and_max(&mut self, value: i32, max: i32) {
        self.value = value;
        self.max = max;
        self.true_zero_scale();
    }

    pub fn set_scale(&mut self, scale: i32) {
        self.scale = scale;
        self.true_zero_scale();
    }

    pub fn value(&self) -> i32 {
        self.scale_value
    }

    fn true_zero_scale(&mut self) {
        let Self {
            max,
            value,
            scale,
            scale_value,
        } = self;

        *scale_value = if *max > 0 {
            (*value * *scale + *max / 2) / *max
        } else {
            0
        };

        if *scale_value == 0 && *value > 0 && *max > 0 {
            *scale_value = 1
        }
    }
}
