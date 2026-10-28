    struct Laptop {
        brand:String,
        quantity:u32,
        price:u32
    }

    impl Laptop {
        fn totalcost(&self)->u32 {
            self.quantity * self.price
        }
    }

    fn main() { 
        let lap1 = Laptop {
            brand:String::from("HP"),
            quantity:3,
            price:650_000
        };
        let lap2 = Laptop {
            brand:String::from("IBM"),
            quantity:3,
            price:755_000
        };
        let lap3 = Laptop {
            brand:String::from("Toshiba"),
            quantity:3,
            price:550_000
        };
        let lap4 = Laptop {
            brand:String::from("Dell"),
            quantity:3,
            price:850_000
        };

        println!(" HP = N{} \n IBM = N{} \n Toshiba = N{} \n Dell = N{}", lap1.totalcost(), lap2.totalcost(), lap3.totalcost(), lap4.totalcost());

        let total = lap1.totalcost() + lap2.totalcost() + lap3.totalcost() + lap4.totalcost();
        println!(" Total of all brands = N{}", total);
    }
    