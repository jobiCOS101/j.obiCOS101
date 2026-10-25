   use std::io::Write;

   fn main () {
    let commissioner = vec!["Aigbogun Alamba Daudu",
                            "Murtala Afeez Bendu",
                            "Okorocha Calistus Ogbona",
                            "Adewale Jimoh Akanbi",
                            "Osazuwa Faith Etieye"];

    let ministry = vec!["Internal Affairs",
                        "Justice",
                        "Defense",
                        "Power & Steel",
                        "Petroleum"];

    let zone = vec!["South West",
                    "North East",
                    "South South",
                    "South West",
                    "South East"];

    let mut file = std::fs::File::create("EFCC.csv").expect("create failed");
    file.write_all("EFCC file\n".as_bytes()).expect("write failed");
    for i in 0..commissioner.len() {
        let record = format!("{}, {}, {}\n", commissioner[i],ministry[i],zone[i]);
        file.write_all(record.as_bytes()).expect("write failed");
        } 
    println!("Data written into file");

   }