//! RaEx is a tool to help you build high performance compute clusters, with which you can run
//! computational tasks that would otherwise be incredibly inefficient on a single system.
pub mod rtrc;

pub fn to_tuple(n: u16) -> Vec<u8> {
    vec![(n >> 8) as u8, n as u8]
}

pub fn tuple_to(tuple: &Vec<u8>) -> u16 {
    (tuple[0] as u16) << 8 | (tuple[1] as u16)
}
