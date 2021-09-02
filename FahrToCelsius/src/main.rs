use std::io;
use std::process::exit;

fn main() {
    println!("Would you like to convert from Fahrenheit or Celsius? (fahr/cels)");
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Unable to read line");

    println!("Enter the temperature that you would like to convert");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Unable to read line");

    let temp : f32 = match temp.trim().parse(){
        Ok(temp) => temp,
        Err(_) => exit(1)
    };

    let mut conv = temp;
    let mut from = String::new();
    let mut to = String::new();

    if choice.trim() == "cels" {
        conv = cels_to_fahr(conv);
        from = String::from("cels");
        to = String::from("fahr");
    } else if choice.trim() == "fahr" {
        conv = fahr_to_cels(conv);
        from = String::from("fahr");
        to = String::from("cels");
    }else{
        println!("{} neither fahr nor cels, exiting",choice);
        exit(1);
    }

    println!("{} {} converted to {} {}",temp,from,conv,to);
}

fn cels_to_fahr(t:f32) -> f32{
    (t * 9.0 / 5.0) + 32.0
}

fn fahr_to_cels(t:f32) -> f32{
    (t-32.0) * 5.0 / 9.0
}
