fn main() {

    let a = factorial(5);
    println!("The value of element is: {}", a);

    let a = fibonacci(10);
    println!("The value of element is: {}", a);

}


fn factorial(n: u32) -> u32 {
    let mut result: u32 = 1;
    for i in 1..n+1 {
        result = result * i;
    }
    result
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }
    fibonacci(n-1) + fibonacci(n-2)
}

