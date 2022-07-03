fn main() {
    let _bank_balancd: i8 = -120; // -128~127
    let _days_of_week: u8 = 7; // 0~255
    
    let _age1: f32 = 0.1 + 0.2; // 0.3
    let _age2: f64 = 0.1 + 0.2; // 0.3..4

    // tuple for (initial, age)
    let _person: (char, u8) = ('j', 67);
    let (_initial, _age) = _person;
    let _age = _person.1;

    // array
    {

        let _a = ['a', 'b', 'c'];
        let _first = _a[0];

        let _b: [i32; 3] = [12, 100, 50];

        let _c = ['a'; 2]; // ['a', 'a']

    }


}
