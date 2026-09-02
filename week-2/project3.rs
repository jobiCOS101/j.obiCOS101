fn main() {
   let p:f64 = 210_000.0;
   let r:f64 = 5.0;
   let n:i32 = 3;

   // depreciation
   let a = p * (1.0 - r / 100.0).powi(n);
   println!("Amount is {}", a);
   let de = a - p;
   println!("depreciation is {}", de);

}
