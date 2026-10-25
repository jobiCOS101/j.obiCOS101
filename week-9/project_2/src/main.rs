    use std::io::Write;

    fn main() {
        let heading = ["Student Name", "Matric Number", "Department", "Level"];
        let std1 = ["Oluchi Mordi", "ACC10211111", "Accounting", "300"];
        let std2 = ["Adams Aliyu", "ECO10110101", "Economics", "100"];
        let std3 = ["Shania Bolade", "CSC10328828", "Computer", "200"];
        let std4 = ["Adekunle Gold", "EEE11020202", "Electrical", "200"];
        let std5 = ["Blanca Edemoh", "MEE10202001", "Mechanical", "100"];

        println!("Student Management Information System");
        println!("{:?}", heading);
        println!("{:?}", std1);
        println!("{:?}", std2);
        println!("{:?}", std3);
        println!("{:?}", std4);
        println!("{:?}", std5);

        let mut file = std::fs::File::create("school.csv").expect("create failed");
        file.write_all("Student Management Information System\n".as_bytes()).expect("write failed");
        let rows = [heading, std1, std2, std3, std4, std5];
        for item in rows {
        let record = format!("{}, {}, {}, {}\n", item[0],item[1],item[2],item[3]);
        file.write_all(record.as_bytes()).expect("write failed");
            }
            
        println!("\nData written to file" );

        }