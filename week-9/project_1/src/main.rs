use std::io::Write;
use std::fs::File;


fn main() {
   let lager :[&str; 6] = ["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
   let stout :[&str; 6] = ["Legend", "Turbo", "Williams", "", "", ""];
   let non_alcoholic :[&str; 6] = ["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz", "", ""];
  
   let mut file = File::create("Drinks.csv").expect("create failed");

   file.write_all("lager, Stout, Alcoholic\n".as_bytes()).expect("write failed");

   let mut index = 0;
    loop{
        file.write_all(lager[index].as_bytes()).expect("write failed");
        file.write_all(",".as_bytes()).expect("write failed");
        file.write_all(stout[index].as_bytes()).expect("write failed");
        file.write_all(",".as_bytes()).expect("write failed");
        file.write_all(non_alcoholic[index].as_bytes()).expect("write fail");
        file.write_all("\n".as_bytes()).expect("write failed");

        index += 1;
        if index == 6{
            break;
        }
    }






}
