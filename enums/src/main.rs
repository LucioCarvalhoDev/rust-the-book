fn main() {



    enum Player {
        Ranger,
        Warrior,
        Mage
    }

    impl Player {
        fn item(&self) -> &str {
            match &self {
                Player::Ranger => "bow",
                Player::Warrior => "mace",
                Player::Mage => "book"
            }
        }

    } 

    let lord03 = Player::Ranger;

    println!("{}", lord03.item());
}

