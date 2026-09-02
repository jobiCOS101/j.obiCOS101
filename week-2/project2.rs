fn main () {
   let sales = [450000.0,1500000.0,750000.0,2850000.0,250000.0];
   let sum: f64 = sales.iter().sum();
   let average = sum / sales.len() as f64;

  println!("sum of the sales = {:.3}", sum);
  println!("average sales = {:.3}", average);
}