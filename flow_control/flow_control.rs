fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("next val is {}", b);

        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);

    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("next val is {}", b)
    } 
}

fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);

    for _i in 2..n {
        // let c = a + b;
        // a = b;
        // b = c;
        let (x, y) = cal(a, b);
        a = x;
        b = y;
        println!("next val is {}", b);
    }
}

fn cal(mut a: u8, mut b: u8) -> (u8, u8) {
    let c = a + b;
    a = b;
    b = c;
    return (a, b);
}

fn main() {
    let n = 10;
    fib_loop(n);
    fib_while(n);
    fib_for(n);
}