pub fn run() -> () {
    assert_eq!((18,25), tuple_operations(5,6));
    string_operations();
    variables();
    is_leap_year(2025);

}

fn string_operations() -> bool {
    let the_string = String::from("BOA!");

    assert_eq!(the_string, "BOA!");
    let generated_string = format!("Hola, {}", get_name(None));
    assert_eq!("Hola, mundo", generated_string);
    let generated_named_string = format!("Hola, {}", get_name(Some("trainee".to_string())));
    assert_eq!("Hola, mundo", generated_named_string);
    let reverse_string: String = generated_named_string.chars().rev().collect();
    assert_eq!("eeniart ,aloH", reverse_string);
   // let reverse_graphemes: String = generated_named_string.graphemes().rev().collect();
    //assert_eq!("eeniart ,aloH", reverse_graphemes);


    true
}

fn get_name(name: Option<String>) -> String {
    match name {
        Some(n) => format!("dear {}", n),
        None => "mundo".to_string()
    }
}

fn variables() -> bool {
    let x: i32 = 5;
    let mut y: i32 = 7;
    y = y * 3;
    assert_eq!(x,5);
    assert_eq!(y,21);
    let original_sum: i32 = x + y;
    {
        let y: i32 = 5;
        let alternative_sum: i32 = x + y ;
        assert_eq!(alternative_sum, 10);
    }

    assert_eq!(original_sum, 26);
    let n: u16 = 38u8 as u16;
    let sum: u32 = n as u32 + 1000000;
    assert_eq!(sum, 1000038);

    true
}

fn tuple_operations(m: i32, n: i32) -> (i32,i32){
    let (x, y) = (3,5);
    let (a, b) = (n, m);
    let (o,p) = (x * a, y * b);
     let (q,..) = (100, 101);
    let [.., r] = [200,201];
    assert_eq!((100,201), (q,r));

    (o,p) 
}

fn is_leap_year(year: u64) -> bool {
    match(year % 4, year % 100, year % 400) {
        (_,_,0 )  => true,
        (_,0,_ )  => false,
    (0,_,_ )  => true,        
    _ => false
    }
}


