fn main() {
    floating_point_types();
    bool_types();
    characters();
    bool_types();
    boxes_type();
    array_types();
    vector_type();
    slice_type();
    string_type();
}

fn floating_point_types() {
    // 31415.926e-4f64
    // integer + fractional + exponent + type suffix
    // integer (required) and need one of the other parts

    assert!((-1. / f32::INFINITY).is_sign_negative());
    assert_eq!(-f32::MIN , f32::MAX);

    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.); // exactly 5.0, per IEEE
    assert_eq!((-1.01f64).floor(), -2.0);
}

fn bool_types() {
    // if x != 0 { ... } correct
    // if x { ... } incorrect
    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);

    // uses a byte for bool value in memory,
    // so you can create a pointer to it
}

fn characters() {
    // char: 32 bit value
    // string: sequence of UTF-8 bytes
    // enclosed in single quotes e.g. '8' or '!'
    assert_eq!('*' as i32, 42);
    assert_eq!('*'.is_alphabetic(), false);

    assert_eq!(std::char::from_digit(2, 10), Some('2'));
}

fn boxes_type() {
    let t = (12, "eggs");
    let _b = Box::new(t);
}

fn array_types() {
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    assert!(sieve[211]);
    assert!(!sieve[9876]);

    let mut chaos = [3, 5, 4, 1, 2];

    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
}

fn vector_type() {
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);

    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);

    fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
        vec![0; rows * cols]
    }

    new_pixel_buffer(1, 1);

    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");
    assert_eq!(pal, vec!["step", "on", "no", "pets"]);

    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);

    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);

    v.push(3);
    assert_eq!(v.len(), 3);
    println!("capacity is now {}", v.capacity());
}

fn slice_type() {
    let v: Vec<f64> = vec!  [0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] =       [0.0, -0.707, -1.0, -0.707];

    let _sv: &[f64] = &v;
    let _sa: &[f64] = &a;

    fn print(n: &[f64]) {
        for elt in n {
            println!("{}",  elt);
        }
    }

    print(&a); // array
    print(&v); // vectors
}

fn string_type() {
    // String literals
    let speech = "\"Ouch!\" said the well.\n";
    println!("{}", speech);

    println!("In the room the women come and go,
        Singing of Mount Abora");

    println!("It was a bright, cold day in April, and \
        there were four of us-\
        more or less.");

    let default_win_install_path = r"C:\Program Files\Gorilla";
    println!("{}", default_win_install_path);

    // let pattern = Regex::new(r"\d+(\,\d+)*");

    println!(r###"
        This raw string started with 'r###"'.
        Therefore it does not end until we reach a quote mark ('"')
        followed immediately by three pound signs ('###'):
    "###);

    // Byte strings
    let method = b"GET"; // &[u8, 3]
    assert_eq!(method, &[b'G', b'E', b'T']);

    let method = br"GET"; // &[u8, 3] raw  
    assert_eq!(method, &[b'G', b'E', b'T']);

    let noodles = "noodles".to_string(); // String
    let _oodles = &noodles[1..]; // &str
    let poodles = "ಠ_ಠ"; // &str

    assert_eq!(poodles.len(), 7);
    assert_eq!(poodles.chars().count(), 3);

    let s = "hello".to_lowercase();
    println!("{}", s);

    // &str -> &[T]
    // String -> Vec<T>
}