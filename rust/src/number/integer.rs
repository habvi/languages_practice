#[allow(dead_code, unused_variables, unused_assignments)]
fn main() {
    // int : default = i32
    {
        let x;
        let y: i32;
        x = 2147483647;
        y = -2147483648;

        let x: i64 = 998244353;
        let y: i64 = 998244353i64;
        let z: i64 = 998244353_i64;
    }
    // reference
    {
        let mut x = 100;

        let p: &i32 = &x;
        assert_eq!(p, &100);

        let p: &mut i32 = &mut x;
        *p = 5;
        assert_eq!(p, &5);
        assert_eq!(x, 5);
    }
    // const
    {
        const MOD: i32 = 998244353;
        const MOD2: i32 = 1_000_000_007;
        const INF: usize = 1e18 as usize;
        const PI: f64 = std::f64::consts::PI; // 3.141592653589793
        const MIN: usize = usize::MIN; // 0
        let max = i64::MAX; // 9223372036854775807
        let x = usize::BITS; // 64
    }
    // binary number
    {
        let base_2 = 0b1110;
        let base_8 = 0o1110;
        let base_16 = 0x1110;
        println!("{:b} {:x} {:o}", 20, 20, 20); // 10100 14 24
    }
    // exponentiation
    {
        let x: i32 = 2;
        let y = x.pow(8);
        let y = 2_i32.pow(10);
    }
}
