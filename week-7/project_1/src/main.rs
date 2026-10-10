use std::io;


fn main() {
    println!("Welcome!\n");
    println!("What do you want to solve?\n");
    println!("1. Area of a Trapezium");
    println!("2. Area of a Rhombus");
    println!("3. Area of a Parallelogram");
    println!("4. Surface area of a Cube");
    println!("5. Volume of a Cylinder\n");
    println!("Please enter 1-5");

     let mut input1 = String::new();
     io::stdin().read_line(&mut input1).expect("Not a valid string");
     let option:i8 = input1.trim().parse().expect("Not a valid number");

     if option == 1{
        trapezium();
     }
     else if option == 2{
        rhombus();
     }
     else if option == 3{
        parallelogram();
     }
     else if option == 4{
        cube();
     }
     else if option == 5{
        cylinder();
     }
     else{
        println!("Enter a value from 1-5");
     }
}


fn trapezium() {
    println!("\nEnter the height");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let height:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the 1st base");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let base1:f64 = input2.trim().parse().expect("Not a valid number");
  
    println!("Enter the 2nd base");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let base2:f64 = input3.trim().parse().expect("Not a valid number");

    let area:f64 = height / 2.0 * (base1 + base2);
    println!("\nThe Area of the Trapezium is {} ", area);
}


fn rhombus() {
    println!("\nEnter the 1st diagonal");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let diagonal1:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the 2nd diagonal");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let diagonal2:f64 = input2.trim().parse().expect("Not a valid number");

    let area:f64 = 0.5 * diagonal1 * diagonal2;
     println!("\nThe Area of the Rhombus is {} ", area);
}

fn parallelogram() {
    println!("\nEnter the base");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let base:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the altitude");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let altitude:f64 = input2.trim().parse().expect("Not a valid number");

    let area:f64 = base * altitude;
     println!("\nThe Area of the Parallelogram is {} ", area);
}

fn cube() {
    println!("\nEnter the side");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let side:f64 = input1.trim().parse().expect("Not a valid number");

    let surface_area:f64 = 6.0 * side * side;
    println!("\nThe Surface area of the Cube is {} ",surface_area);
}

fn cylinder() {
    println!("\nEnter the height");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let height:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the radius");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let radius:f64 = input2.trim().parse().expect("Not a valid number");
    
    let volume:f64 = 3.142 * radius * radius * height;
    println!("\nThe Volume of the Cylinder is {:.2} ", volume);
}