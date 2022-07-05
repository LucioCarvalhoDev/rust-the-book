fn main() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        
        fn can_contain(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size
            }
        }
    }

    let rec1 = Rectangle {
        width: 40,
        height: 120
    };
    
    let rec2 = Rectangle {
        width: 35,
        height: 10
    };

    let rec3 = Rectangle::square(90);

    let recs = [rec1, rec2, rec3];

    {
    let mut i = 1;
    for rec in recs {
        println!("rec{}", i);
        println!("{:#?}", rec);
        println!("area: {}", rec.area());

        i = i + 1;
    }

    }
}
