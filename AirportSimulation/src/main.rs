use crate::Simulation::AirportSimulation;

mod Simulation;

fn main() {

    let config_files : [&str;5]  = ["./simulation_configs/busier.data",
        "./simulation_configs/busier_and_save_fuel.data",
        "./simulation_configs/current.data",
        "./simulation_configs/safer.data",
        "./simulation_configs/save_fuel.data"];

    for f  in config_files{
        let sim = AirportSimulation::new(f);
    }

}
