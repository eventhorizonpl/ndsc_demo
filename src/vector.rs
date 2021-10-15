use crate::operation::{Operation2D, Operation3D, Operation4D};

#[derive(Debug)]
pub struct Vector2D {
    data: Vec<Vec<u8>>,
}

impl Vector2D {
    pub fn new() -> Self {
        Self {
            data: vec![vec![0; 256]; 256],
        }
    }

    pub fn decode(&self, operation_2d: &Operation2D) -> u8 {
        self.data[*operation_2d.d1() as usize][*operation_2d.d2() as usize]
    }

    pub fn encode(&self, number: &u8) -> Operation2D {
        let d1: u8 = rand::random::<u8>();
        let d2: u8 = self.data[d1 as usize]
            .iter()
            .position(|&n| n == *number)
            .unwrap() as u8;
        let s: u8 = rand::random::<u8>();

        Operation2D::new(d1, d2, s)
    }

    pub fn init(&mut self, password: &[u8]) {
        let mut new_vec: Vec<u8>;
        let mut password_shift: usize = 0;
        let mut shift: u16 = 0;

        for d2 in self.data.iter_mut() {
            shift += password[password_shift] as u16;

            if shift >= 255 {
                shift %= 255;
            }

            new_vec = create_vec(&shift);

            *d2 = new_vec;

            password_shift += 1;

            if password_shift >= 64 {
                password_shift = 0;
            }
        }
    }

    pub fn shift(&mut self, operation_2d: &Operation2D) {
        for d2 in self.data.iter_mut() {
            let shifted_vec: Vec<u8> = shift_vec(d2, operation_2d.s());

            *d2 = shifted_vec;
        }
    }
}

#[derive(Debug)]
pub struct Vector3D {
    data: Vec<Vec<Vec<u8>>>,
}

impl Vector3D {
    pub fn new() -> Self {
        Self {
            data: vec![vec![vec![0; 256]; 256]; 256],
        }
    }

    pub fn decode(&self, operation_3d: &Operation3D) -> u8 {
        self.data[*operation_3d.d1() as usize][*operation_3d.d2() as usize]
            [*operation_3d.d3() as usize]
    }

    pub fn encode(&self, number: &u8) -> Operation3D {
        let d1: u8 = rand::random::<u8>();
        let d2: u8 = rand::random::<u8>();
        let d3: u8 = self.data[d1 as usize][d2 as usize]
            .iter()
            .position(|&n| n == *number)
            .unwrap() as u8;
        let s: u8 = rand::random::<u8>();

        Operation3D::new(d1, d2, d3, s)
    }

    pub fn init(&mut self, password: &[u8]) {
        let mut new_vec: Vec<u8>;
        let mut password_shift: usize = 0;
        let mut shift: u16 = 0;

        for d2 in self.data.iter_mut() {
            for d3 in d2 {
                shift += password[password_shift] as u16;

                if shift >= 255 {
                    shift %= 255;
                }

                new_vec = create_vec(&shift);

                *d3 = new_vec;

                password_shift += 1;

                if password_shift >= 64 {
                    password_shift = 0;
                }
            }
        }
    }

    pub fn shift(&mut self, operation_3d: &Operation3D) {
        for d2 in self.data.iter_mut() {
            for d3 in d2 {
                let shifted_vec: Vec<u8> = shift_vec(d3, operation_3d.s());

                *d3 = shifted_vec;
            }
        }
    }
}

#[derive(Debug)]
pub struct Vector4D {
    data: Vec<Vec<Vec<Vec<u8>>>>,
}

impl Vector4D {
    pub fn new() -> Self {
        Self {
            data: vec![vec![vec![vec![0; 256]; 256]; 256]; 256],
        }
    }

    pub fn decode(&self, operation_4d: &Operation4D) -> u8 {
        self.data[*operation_4d.d1() as usize][*operation_4d.d2() as usize]
            [*operation_4d.d3() as usize][*operation_4d.d4() as usize]
    }

    pub fn encode(&self, number: &u8) -> Operation4D {
        let d1: u8 = rand::random::<u8>();
        let d2: u8 = rand::random::<u8>();
        let d3: u8 = rand::random::<u8>();
        let d4: u8 = self.data[d1 as usize][d2 as usize][d3 as usize]
            .iter()
            .position(|&n| n == *number)
            .unwrap() as u8;
        let s: u8 = rand::random::<u8>();

        Operation4D::new(d1, d2, d3, d4, s)
    }

    pub fn init(&mut self, password: &[u8]) {
        let mut new_vec: Vec<u8>;
        let mut password_shift: usize = 0;
        let mut shift: u16 = 0;

        for d2 in self.data.iter_mut() {
            for d3 in d2 {
                for d4 in d3 {
                    shift += password[password_shift] as u16;

                    if shift >= 255 {
                        shift %= 255;
                    }

                    new_vec = create_vec(&shift);

                    *d4 = new_vec;

                    password_shift += 1;

                    if password_shift >= 64 {
                        password_shift = 0;
                    }
                }
            }
        }
    }

    pub fn shift(&mut self, operation_4d: &Operation4D) {
        for d2 in self.data.iter_mut() {
            for d3 in d2 {
                for d4 in d3 {
                    let shifted_vec: Vec<u8> = shift_vec(d4, operation_4d.s());

                    *d4 = shifted_vec;
                }
            }
        }
    }
}

fn create_vec(shift: &u16) -> Vec<u8> {
    let mut new_vec: Vec<u8> = vec![0; 256];
    let mut start_value: u16 = *shift;
    for val in new_vec.iter_mut() {
        *val = start_value as u8;

        start_value += 1;

        if start_value >= 256 {
            start_value = 0;
        }
    }

    new_vec
}

fn shift_vec(old_vec: &[u8], shift: &u8) -> Vec<u8> {
    let mut new_vec: Vec<u8> = Vec::new();
    let mut i: u16 = *shift as u16;

    for _n in 0..256 {
        new_vec.push(old_vec[i as usize]);

        i += 1;

        if i >= 256 {
            i = 0;
        }
    }

    new_vec
}
