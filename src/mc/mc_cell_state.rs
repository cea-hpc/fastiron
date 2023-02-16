#[derive(Debug)]
pub struct MCCellState {
    material: u32, // gid of material; usize?
    total: Vec<f64>, // energy groups
    volume: f64, // cell volume
    cell_number_density: f64,
    id: usize,
    source_tally: usize,
}