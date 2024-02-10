use libccanvas::{
    bindings::{Colour, Discriminator},
    client::Client,
    features::common::{Direction, Rect},
};
use serde::Deserialize;

use crate::{Border, BorderSet, Constraint};

#[derive(Deserialize)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[serde(tag = "type")]
pub enum Layout {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "single")]
    Single {
        discrim: Option<Discriminator>,
        border: Option<Border>,
    },
    #[serde(rename = "split horizontal")]
    SplitHorizontal {
        left_constraint: Constraint,
        left: Box<Layout>,
        right_constraint: Constraint,
        right: Box<Layout>,
    },
    #[serde(rename = "split vertical")]
    SplitVertical {
        top_constraint: Constraint,
        top: Box<Layout>,
        bottom_constraint: Constraint,
        bottom: Box<Layout>,
    },
}

impl Default for Layout {
    fn default() -> Self {
        Self::None
    }
}

impl Layout {
    pub fn single(discrim: Option<Discriminator>, border: Option<Border>) -> Self {
        Self::Single { discrim, border }
    }

    pub fn horizontal(
        left: Layout,
        right: Layout,
        left_constraint: Constraint,
        right_constraint: Constraint,
    ) -> Self {
        Self::SplitHorizontal {
            left: left.into(),
            right: right.into(),
            left_constraint,
            right_constraint,
        }
    }

    pub fn vertical(
        top: Layout,
        bottom: Layout,
        top_constraint: Constraint,
        bottom_constraint: Constraint,
    ) -> Self {
        Self::SplitVertical {
            top: top.into(),
            bottom: bottom.into(),
            top_constraint,
            bottom_constraint,
        }
    }
}

impl Layout {
    /// add an item, returns whether layout is updated
    pub fn add(
        &mut self,
        at: &[Direction],
        split: &Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Option<Discriminator>,
        border: Option<Border>,
    ) -> bool {
        if at.is_empty() {
            match split {
                Direction::Up => {
                    *self = Self::vertical(
                        Layout::single(component, border),
                        std::mem::take(self),
                        constraint_1,
                        constraint_2,
                    )
                }
                Direction::Down => {
                    *self = Self::vertical(
                        std::mem::take(self),
                        Layout::single(component, border),
                        constraint_1,
                        constraint_2,
                    )
                }
                Direction::Left => {
                    *self = Self::horizontal(
                        Layout::single(component, border),
                        std::mem::take(self),
                        constraint_1,
                        constraint_2,
                    )
                }
                Direction::Right => {
                    *self = Self::horizontal(
                        std::mem::take(self),
                        Layout::single(component, border),
                        constraint_1,
                        constraint_2,
                    )
                }
            }

            return true;
        }

        match self {
            Self::SplitHorizontal { left, right, .. }
                if matches!(at[0], Direction::Left | Direction::Right) =>
            {
                if at[0] == Direction::Left {
                    left.add(
                        &at[1..],
                        split,
                        constraint_1,
                        constraint_2,
                        component,
                        border,
                    )
                } else {
                    right.add(
                        &at[1..],
                        split,
                        constraint_1,
                        constraint_2,
                        component,
                        border,
                    )
                }
            }
            Self::SplitVertical { top, bottom, .. }
                if matches!(at[0], Direction::Up | Direction::Down) =>
            {
                if at[0] == Direction::Up {
                    top.add(
                        &at[1..],
                        split,
                        constraint_1,
                        constraint_2,
                        component,
                        border,
                    )
                } else {
                    bottom.add(
                        &at[1..],
                        split,
                        constraint_1,
                        constraint_2,
                        component,
                        border,
                    )
                }
            }
            _ => false,
        }
    }

    /// remove an item, returns whether layout is updated
    pub fn remove(&mut self, at: &[Direction]) -> bool {
        if at.is_empty() {
            *self = Self::None;
            return true;
        }

        if at.len() == 1 {
            match self {
                Self::SplitHorizontal { left, right, .. }
                    if matches!(at[0], Direction::Left | Direction::Right) =>
                {
                    if at[0] == Direction::Left {
                        *self = std::mem::take(right);
                    } else {
                        *self = std::mem::take(left);
                    }

                    return true;
                }
                Self::SplitVertical { top, bottom, .. }
                    if matches!(at[0], Direction::Up | Direction::Down) =>
                {
                    if at[0] == Direction::Up {
                        *self = std::mem::take(bottom);
                    } else {
                        *self = std::mem::take(top);
                    }

                    return true;
                }
                _ => {}
            }
        }

        match self {
            Self::SplitHorizontal { left, right, .. }
                if matches!(at[0], Direction::Left | Direction::Right) =>
            {
                if at[0] == Direction::Left {
                    left.remove(&at[1..])
                } else {
                    right.remove(&at[1..])
                }
            }
            Self::SplitVertical { top, bottom, .. }
                if matches!(at[0], Direction::Up | Direction::Down) =>
            {
                if at[0] == Direction::Up {
                    top.remove(&at[1..])
                } else {
                    bottom.remove(&at[1..])
                }
            }
            _ => false,
        }
    }

    pub fn set(&mut self, at: &[Direction], state: Layout) -> bool {
        if at.is_empty() {
            *self = state;
            return true;
        }

        match self {
            Self::SplitHorizontal { left, right, .. }
                if matches!(at[0], Direction::Left | Direction::Right) =>
            {
                if at[0] == Direction::Left {
                    left.set(&at[1..], state)
                } else {
                    right.set(&at[1..], state)
                }
            }
            Self::SplitVertical { top, bottom, .. }
                if matches!(at[0], Direction::Up | Direction::Down) =>
            {
                if at[0] == Direction::Up {
                    top.set(&at[1..], state)
                } else {
                    bottom.set(&at[1..], state)
                }
            }
            _ => false,
        }
    }

    pub fn get(&self, at: &[Direction]) -> Option<&Self> {
        if at.is_empty() {
            return Some(self);
        }

        match self {
            Self::SplitHorizontal { left, right, .. }
                if matches!(at[0], Direction::Left | Direction::Right) =>
            {
                if at[0] == Direction::Left {
                    left.get(&at[1..])
                } else {
                    right.get(&at[1..])
                }
            }
            Self::SplitVertical { top, bottom, .. }
                if matches!(at[0], Direction::Up | Direction::Down) =>
            {
                if at[0] == Direction::Up {
                    top.get(&at[1..])
                } else {
                    bottom.get(&at[1..])
                }
            }
            _ => None,
        }
    }

    pub fn areas(&self, screen: Rect, client: &Client) -> Vec<(Rect, Discriminator)> {
        let mut areas: Vec<(Rect, Discriminator)> = Vec::new();

        match self {
            Self::None => {}
            Self::Single {
                discrim,
                border: Some(border),
            } => {
                if screen.width > 1 && screen.height > 1 {
                    let borderset: BorderSet = (&border.r#type).into();

                    client.setcharcoloured(
                        screen.x,
                        screen.y,
                        borderset.topleft,
                        border.colour,
                        Colour::Reset,
                    );
                    client.setcharcoloured(
                        screen.x + screen.width - 1,
                        screen.y,
                        borderset.topright,
                        border.colour,
                        Colour::Reset,
                    );
                    client.setcharcoloured(
                        screen.x + screen.width - 1,
                        screen.y + screen.height - 1,
                        borderset.bottomright,
                        border.colour,
                        Colour::Reset,
                    );
                    client.setcharcoloured(
                        screen.x,
                        screen.y + screen.height - 1,
                        borderset.bottomleft,
                        border.colour,
                        Colour::Reset,
                    );

                    (screen.x + 1..screen.x + screen.width - 1).for_each(|x| {
                        client.setcharcoloured(
                            x,
                            screen.y,
                            borderset.top,
                            border.colour,
                            Colour::Reset,
                        );
                        client.setcharcoloured(
                            x,
                            screen.y + screen.height - 1,
                            borderset.bottom,
                            border.colour,
                            Colour::Reset,
                        );
                    });

                    (screen.y + 1..screen.y + screen.height - 1).for_each(|y| {
                        client.setcharcoloured(
                            screen.x,
                            y,
                            borderset.left,
                            border.colour,
                            Colour::Reset,
                        );
                        client.setcharcoloured(
                            screen.x + screen.width - 1,
                            y,
                            borderset.right,
                            border.colour,
                            Colour::Reset,
                        );
                    });

                    if let Some(discrim) = discrim {
                        if screen.width > 2 && screen.height > 2 {
                            areas.push((
                                Rect::new(
                                    screen.x + 1,
                                    screen.y + 1,
                                    screen.width - 2,
                                    screen.height - 2,
                                ),
                                discrim.clone(),
                            ))
                        } else {
                            areas.push((Rect::new(0, 0, 0, 0), discrim.clone()))
                        }
                    }
                } else if let Some(discrim) = discrim {
                    areas.push((Rect::new(0, 0, 0, 0), discrim.clone()))
                }
            }
            Self::Single { discrim, .. } => {
                if let Some(discrim) = discrim {
                    areas.push((screen, discrim.clone()))
                }
            }
            Self::SplitVertical {
                top_constraint,
                top,
                bottom_constraint,
                bottom,
            } => {
                let top_height = top_constraint.eval(screen.height);
                let bottom_height = bottom_constraint
                    .eval(screen.height)
                    .min(screen.height - top_height);

                areas.extend(top.areas(
                    Rect::new(screen.x, screen.y, screen.width, top_height),
                    client,
                ));
                areas.extend(bottom.areas(
                    Rect::new(screen.x, screen.y + top_height, screen.width, bottom_height),
                    client,
                ));
            }
            Self::SplitHorizontal {
                left_constraint,
                left,
                right_constraint,
                right,
            } => {
                let left_width = left_constraint.eval(screen.width);
                let right_width = right_constraint
                    .eval(screen.width)
                    .min(screen.width - left_width);

                areas.extend(left.areas(
                    Rect::new(screen.x, screen.y, left_width, screen.height),
                    client,
                ));
                areas.extend(right.areas(
                    Rect::new(screen.x + left_width, screen.y, right_width, screen.height),
                    client,
                ));
            }
        }

        areas
    }

    pub fn components(&self) -> Vec<Discriminator> {
        let mut out = Vec::new();

        match &self {
            Self::None => {}
            Self::Single { discrim, .. } => {
                if let Some(discrim) = discrim {
                    out.push(discrim.clone())
                }
            }
            Self::SplitHorizontal { left, right, .. } => {
                out.append(&mut left.components());
                out.append(&mut right.components());
            }
            Self::SplitVertical { top, bottom, .. } => {
                out.append(&mut top.components());
                out.append(&mut bottom.components());
            }
        }

        out
    }
}
