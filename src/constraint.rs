use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Constraint {
    base: ConstraintVariant,
    offset_pos: Option<Box<Constraint>>,
    offset_neg: Option<Box<Constraint>>,
}

impl Constraint {
    pub fn new(
        base: ConstraintVariant,
        offset_pos: Option<Constraint>,
        offset_neg: Option<Constraint>,
    ) -> Self {
        Self {
            base,
            offset_pos: offset_pos.map(Box::new),
            offset_neg: offset_neg.map(Box::new),
        }
    }

    pub fn eval(&self, length: u32) -> u32 {
        let base = self.base.eval(length);
        (base
            + self
                .offset_pos
                .as_ref()
                .map(|offset| offset.eval(base))
                .unwrap_or(0))
        .saturating_sub(
            self.offset_neg
                .as_ref()
                .map(|offset| offset.eval(base))
                .unwrap_or(0),
        )
    }
}

#[derive(Deserialize)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[serde(tag = "type")]
pub enum ConstraintVariant {
    #[serde(rename = "max")]
    Max { value: u32 },
    #[serde(rename = "min")]
    Min { value: u32 },
    #[serde(rename = "length")]
    Length { value: u32 },
    #[serde(rename = "percentage")]
    Percentage { value: u32 },
}

impl ConstraintVariant {
    pub fn max(value: u32) -> Self {
        Self::Max { value }
    }

    pub fn min(value: u32) -> Self {
        Self::Min { value }
    }

    pub fn length(value: u32) -> Self {
        Self::Length { value }
    }

    pub fn percentage(value: u32) -> Self {
        Self::Percentage { value }
    }

    pub fn eval(&self, length: u32) -> u32 {
        match self {
            Self::Max { value } => length.min(*value),
            Self::Min { value } if *value <= length => length,
            Self::Min { .. } => 0,
            Self::Length { value } if *value <= length => *value,
            Self::Length { .. } => 0,
            Self::Percentage { value } => {
                (((length * value) as f32 / 100_f32).round() as u32).min(length)
            }
        }
    }
}
