use std::ops::Add;

pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    // associated type
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Millimeters(u32);

pub struct Meters(u32);

impl Millimeters {
    pub fn new(v: u32) -> Millimeters {
        Millimeters(v)
    }
}

impl Meters {
    pub fn new(v: u32) -> Meters {
        Meters(v)
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}
