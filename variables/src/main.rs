fn main() {
    // shadowing
    let x = 5; // 5
    let x = 6; // 6
    
    {
        let x = x*2; // 12
    }

    x; // 6
    
    // const
    const Y: u32 = 100; // must be typed
}

