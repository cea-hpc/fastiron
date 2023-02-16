/// Structure used to represent a cell's state, i.e.
/// properties relevant to the simulation.
#[derive(Debug)]
pub struct MCCellState {
    material: u32,   // gid of material; usize?
    total: Vec<f64>, // energy groups
    volume: f64,     // cell volume
    cell_number_density: f64,
    id: usize,
    source_tally: usize,
}
