fn main() {
    let s1 = String::from("Jhon");
    let r1 = &s1;

    do_nothing(r1);
    println!("{}", r1);

}

fn do_nothing(_text: &String) {}
