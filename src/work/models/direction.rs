pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn timeline_class(&self) -> &str {
        match self {
            Direction::Left => "timeline-start md:text-end",
            Direction::Right => "timeline-end md:text-start",
        }
    }

    pub fn from_index(index: usize) -> Self {
        if index % 2 == 0 {
            Self::Left
        } else {
            Self::Right
        }
    }
}
