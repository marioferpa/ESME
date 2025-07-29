// With a lot of help from https://gamedevelopment.tutsplus.com/tutorials/simulate-tearable-cloth-and-ragdolls-with-simple-verlet-integration--gamedev-519
// And https://toqoz.fyi/game-rope.html

// TODO Is this still happening?
// Problem, maybe: The simulation seems to be idle for the two first frames

use bevy::prelude::*;
use crate::{ components, resources, solar_wind, spacecraft };

use uom::si::*;

pub mod acceleration_vector;
pub mod force_vector;
pub mod position_vector;
pub mod runge_kutta_object; // Test
pub mod velocity_vector;
pub mod verlet_object;

// All operations in this plugin should be done in physical units. Get rid of
// pixels in verlets. Graphics.rs should then translate distances to pixels when
// needed.

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                update_center_of_mass
            )
        ;
    }
}


fn update_center_of_mass (
    simulation_parameters:     Res<resources::SimulationParameters>,
    mass_query: Query<
        (&Transform, &components::Mass), 
        Without<spacecraft::center_mass::CenterOfMass>
    >,
    mut com_query: Query<
        &mut Transform, 
        With<spacecraft::center_mass::CenterOfMass>
    >, 
) {

    let mut total_mass:     f32 = 0.0;  // In this particular case I don't think I should use physical units.
                                        // Transform will be in pixels, and mass units are cancelled out.
    let mut center_mass_x:  f32 = 0.0;
    let mut center_mass_y:  f32 = 0.0;

    for (transform, object_mass) in mass_query.iter() {
        total_mass    += object_mass.0.value as f32; 
        center_mass_x += transform.translation.x * object_mass.0.value as f32;
        center_mass_y += transform.translation.y * object_mass.0.value as f32;
    }

    if simulation_parameters.debug {
        println!(
            "Total mass: {} | Center of mass: ({},{})", 
            total_mass, center_mass_x, center_mass_y
        );
    }

    let mut com_transform = com_query.single_mut();

    com_transform.translation.x = center_mass_x;
    com_transform.translation.y = center_mass_y;
}


#[allow(non_snake_case)]
pub fn coulomb_force_per_meter( 
    solar_wind:         &Res<solar_wind::SolarWind>, 
    spacecraft:         &Res<spacecraft::SpacecraftParameters>,
) -> uom::si::f64::RadiantExposure {    // Radiant exposure is [mass][time]⁻²

    // First: r_0, distance at which the potential vanishes
    let r0_numerator    = resources::EPSILON_0 * solar_wind.T_e;
    let r0_denominator  = solar_wind.n_0 * resources::Q_E * resources::Q_E; 
    let r_0             = 2.0 * (r0_numerator / r0_denominator).sqrt();    

    // Second: r_s, stopping distance of protons
    let exp_numerator   = resources::M_PROTON * solar_wind.velocity * solar_wind.velocity * (r_0 / spacecraft.tether_radius).ln();
    let exp_denominator = resources::Q_E * spacecraft.tether_potential; 
    let exp             = (exp_numerator / exp_denominator).exp();  
    let rs_denominator  = (exp.value - 1.0).sqrt();
    let r_s             = r_0 / rs_denominator;

    // Third: force per unit length
    let K = 3.09;   // Empirical, from Monte Carlo sims, I need to calculate this myself somehow.

    let force_per_unit_length = r_s * K * resources::M_PROTON * solar_wind.n_0 * solar_wind.velocity * solar_wind.velocity;

    //println!("{}: {:?}", "Force per meter", force_per_unit_length); 

    return force_per_unit_length;
}
