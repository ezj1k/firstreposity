fn main() {
    // const, tuple, arrays
    const MAX_VALUE: u32 =1_000_000;
    println!("Info {}", MAX_VALUE);

    let user_alex: (i8, bool, f32, char) = (42, true, 1.86, 'R');
    println!("Info {}", user_alex.0); //42

    let mut nums: [i8; 6]= [1,2,3,4,5,6];
    nums[0]=10;
    println!("info {}", nums[0]);
}
