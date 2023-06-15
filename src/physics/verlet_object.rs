use bevy::prelude::*;

// The problem with elements going to 35000 pixels has to be here, the units must be wrong
// initially or something.

#[derive(Component, Debug )]
pub struct VerletObject { 
    pub previous_coordinates:   super::position_vector::PositionVector, 
    pub current_coordinates:    super::position_vector::PositionVector,
    pub is_deployed:            bool,
    // Test, not sure about this
    pub current_force:          super::force_vector::ForceVector, 
}

impl VerletObject {

    pub fn correct_current_coordinates(&mut self, correction_vector: super::position_vector::PositionVector) {
        //self.current_coordinates = self.current_coordinates.add(correction_vector); //Check if add works as you think)
        let current_coordinates = self.current_coordinates.clone();
        let new_coordinates = current_coordinates + correction_vector;
        self.current_coordinates = new_coordinates;
    }

    /// Previous position if forgotten, current coordinates become previous coordinates, and next coordinates become current coordinates.
    pub fn update_coordinates(&mut self, next_coordinates: super::position_vector::PositionVector) {
        let current_coordinates = self.current_coordinates.clone();
        self.previous_coordinates = current_coordinates;
        self.current_coordinates  = next_coordinates;
    }

}
