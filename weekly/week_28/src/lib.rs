pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;

use crate::day_5::ex1_synthese::*;

pub fn populate(entity_nbr: usize, hot_data: &mut HotData) {
    
    for _ in 0..entity_nbr {
        let tab = [5.55; 3];
        hot_data.forces.push(tab);
        hot_data.positions.push(tab);
        hot_data.velocities.push(tab);
    }
}