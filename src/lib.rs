//! RaEx is a tool to help you build high performance compute clusters, with which you can run
//! computational tasks that would otherwise be incredibly inefficient on a single system.
pub mod rtrc;

pub fn coord_vec(i: u16, j: u16) -> Vec<u8> {
    vec![(i >> 8) as u8, i as u8, (j >> 8) as u8, j as u8]
}

pub fn vec_coord(vec: &Vec<u8>) -> (u16, u16) {
    (
        (vec[0] as u16) << 8 | (vec[1] as u16),
        (vec[2] as u16) << 8 | (vec[3] as u16),
    )
}
