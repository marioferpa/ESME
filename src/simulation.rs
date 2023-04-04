// Move the simulation plugin and the resource. Leave physics.rs only with use position_vector, use etc

mod verlet_simulation;

pub struct SimulationPlugin;   // Plugins are structs, therefore they can hold data!

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(Self::update_esail_voltage)         // "Charges" the sail with up to the chosen potential
            .add_system(Self::verlet_simulation)            // Calculates new positions
            .add_system(Self::update_center_of_mass)        // Updates position of the center of mass
            ;
    }
}
