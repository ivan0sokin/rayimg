#[derive(Debug, Clone, Copy, Default)]
pub struct Interval {
    pub min: f64,
    pub max: f64
}

impl Interval {
    pub fn new() -> Self {
        Self {
            min: 0.0,
            max: 0.0
        }
    }

    pub fn limits(min: f64, max: f64) -> Self {
        Self {
            min,
            max
        }
    }

    pub fn unite(first: Interval, second: Interval) -> Self {
        Self {
            min: first.min.min(second.min),
            max: first.max.max(second.max)
        }
    }

    pub fn len(&self) -> f64 {
        self.max - self.min
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta * 0.5;
        Self {
            min: self.min - padding,
            max: self.max + padding
        }
    }
}