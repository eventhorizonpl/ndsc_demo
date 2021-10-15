#[derive(Debug)]
pub struct Operation2D {
    d1: u8,
    d2: u8,
    s: u8,
}

impl Operation2D {
    pub fn new(d1: u8, d2: u8, s: u8) -> Self {
        Self { d1, d2, s }
    }

    pub fn d1(&self) -> &u8 {
        &self.d1
    }

    pub fn d2(&self) -> &u8 {
        &self.d2
    }

    pub fn s(&self) -> &u8 {
        &self.s
    }
}

#[derive(Debug)]
pub struct Operation3D {
    d1: u8,
    d2: u8,
    d3: u8,
    s: u8,
}

impl Operation3D {
    pub fn new(d1: u8, d2: u8, d3: u8, s: u8) -> Self {
        Self { d1, d2, d3, s }
    }

    pub fn d1(&self) -> &u8 {
        &self.d1
    }

    pub fn d2(&self) -> &u8 {
        &self.d2
    }

    pub fn d3(&self) -> &u8 {
        &self.d3
    }

    pub fn s(&self) -> &u8 {
        &self.s
    }
}

#[derive(Debug)]
pub struct Operation4D {
    d1: u8,
    d2: u8,
    d3: u8,
    d4: u8,
    s: u8,
}

impl Operation4D {
    pub fn new(d1: u8, d2: u8, d3: u8, d4: u8, s: u8) -> Self {
        Self { d1, d2, d3, d4, s }
    }

    pub fn d1(&self) -> &u8 {
        &self.d1
    }

    pub fn d2(&self) -> &u8 {
        &self.d2
    }

    pub fn d3(&self) -> &u8 {
        &self.d3
    }

    pub fn d4(&self) -> &u8 {
        &self.d4
    }

    pub fn s(&self) -> &u8 {
        &self.s
    }
}
