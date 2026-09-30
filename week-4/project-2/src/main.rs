use std::io;

fn main(){
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Ener your experience category(experienced or not experienced");
    io::stdin().read_line(&mut input1).expect("Not a valid entry");
    let experience = input1.trim().to_lowercase();

    println!("  Enter your age.");
    io::stdin().read_line(&mut input2).expect("Not a valid entry");
    let age:u8 = input2.trim().parse().expect("Not a valid number");

    if experience == "experienced" {
        if age >= 40 {
            println!("Your annual incentive is 1,560,000");
        } else if 30 >= age && age <= 39{
            println!("Your annual incentive is 1,480,000");
        } else if age <= 29 {
            println!("Your annual incentive is 1,380,000");
        }
    } else if experience == "no experience"{
        println!("Your annual incentive is 100,000");
    } else {
        println!("invalid entry. Enter either experienced or no experience");
    }

}