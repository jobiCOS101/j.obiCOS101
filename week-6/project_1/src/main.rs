use std::io;


fn main() {
    // Display of menu to customer
    println!("\nWelcome to KOSI EATS!\n");
    println!("Here is our MENU!\n");
    println!("A. Poundo/Edinkaiko Soup");
    println!("B. Fried Rice & Chicken");
    println!("C. Amala & Ewedu Soup");
    println!("D. Eba & Egusi Soup");
    println!("E. White Rice & Stew\n");
    println!("What would you like to order?");

    let mut amount = 0;

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Select from A to E");
    let mut choice1 = input1.trim().to_uppercase();

    if choice1 == "A"{
        println!("You have chosen Poundo/Edinkaiko Soup for N3200\n");
        println!("How many portions do you want?");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Enter a definite number");
        let mut quantity:i32 = input2.trim().parse().expect("Enter a definite Number");
        amount += quantity * 3200

    }else if choice1 == "B"{
        println!("You have chosen Fried Rice & Chicken for N3000\n");
        println!("How many portions do you want?");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Enter a definite number");
        let mut quantity:i32 = input2.trim().parse().expect("Enter a definite Number");
        amount += quantity * 3000

    }else if choice1 == "C"{
        println!("You have chosen Amala & Ewedu Soup for N2500\n");
        println!("How many portions do you want?");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Enter a definite number");
        let mut quantity:i32 = input2.trim().parse().expect("Enter a definite Number");
        amount += quantity * 2500

    }else if choice1 == "D"{
        println!("You have chosen Eba & Egusi Soup for N2000\n");
        println!("How many portions do you want?");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Enter a definite number");
        let mut quantity:i32 = input2.trim().parse().expect("Enter a definite Number");
        amount += quantity * 2000

    }else if choice1 == "E"{
        println!("You have chosen White Rice & Stew for N2500\n");
        println!("How many portions do you want?");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Enter a definite number");
        let mut quantity:i32 = input2.trim().parse().expect("Enter a definite Number");
        amount += quantity * 2500
    }else{
        println!("Please enter choices from A to E \n");

    }
    if amount > 10_000{
        let bill = (amount * 19) / 20;
        println!("Since your bill surpasses the N10000 mark, you have recieved a discount of 5% ");
        println!("Your final bill is {}", bill);
    } else{
        println!("Your final bill is {}", amount);
    }
}
