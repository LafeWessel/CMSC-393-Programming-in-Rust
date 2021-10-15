use crate::simulation::AirportSimulation;

mod simulation;

fn main() {

    let config_files : [&str;5]  = ["./simulation_configs/busier.data",
        "./simulation_configs/busier_and_save_fuel.data",
        "./simulation_configs/current.data",
        "./simulation_configs/safer.data",
        "./simulation_configs/save_fuel.data"];

    println!("Running simulations");
    for f  in config_files{
        let mut sim_1 = AirportSimulation::new(f,1);
        sim_1.run_simulation();

        let mut sim_2 = AirportSimulation::new(f,2);
        sim_2.run_simulation();
    }

}
