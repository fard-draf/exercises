

pub const SIZE_ENTITY_NUMBER: usize = 10_000_000;


#[derive(Debug)]
pub struct HotData {
    pub positions: Vec<[f64;3]>,
    pub velocities: Vec<[f64;3]>,
    pub forces: Vec<[f64;3]>,
}



impl HotData {
    pub fn new(capacity: usize) -> Self {
        Self {
            positions: Vec::with_capacity(capacity),
            velocities: Vec::with_capacity(capacity),
            forces: Vec::with_capacity(capacity),
        }
    }
}


#[repr(C)]
pub struct ColdData {
    pub id: Vec<u64>,
    pub name: Vec<String>,
    pub mass: Vec<f64>
}

