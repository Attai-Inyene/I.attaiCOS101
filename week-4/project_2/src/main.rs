use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\n--incentives database--",);

    println!("Enter age");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let age:f32 = input1.trim().parse().expect("Not  valid number");

    println!("are you experienced? yes or no?.");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let experience = input2.trim();
    
    if experience == "no" {
            println!("Your incentive is 100000");
        }
          else if experience == "yes" {
        }selse if age >= 40.0{
            println!("Your incentives is 1560000");
        }else if age >= 30.0{
            println!("your incentives is 1480000");
        }else if age >= 28.0{
            println!("your incentive is undetermined");
        }else if age > 28.0{
            println!("your incentive is 1300000");
    }
}
