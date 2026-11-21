fn main() {
    println!("Hello, world!");

    let mut v : Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    println!("{:?}", v);

    let v2 = vec![1, 2, 3, 4, 5];
    println!("{:?}", v2);

    let third: &i32 = &v2[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }


    let first = &v2[0];
    println!("The first element is {}", first);

    // v2.push(6);

    for i in &v2 {
        println!("{}", i);
    }

    let mut v = vec![1,2,3,4,5];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in &row {
        println!("{:?}", i);
    }
}
