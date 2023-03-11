fn main() {

    println!("loop = {}", loop_fn(2));
    println!("while = {}", while_fn(2));


    println!("for = {}", for_fn());

    let u: u32 = 7;
    for number in (1..u).rev() {
        println!("number = {}", number);
    }

    println!("---------");
    for number in 1..u {
        println!("number = {}", number);
    }
}

fn for_fn() -> u32 {
    println!("----- start for -------");
    let _a = [12, 12, 234, 42, 23 ,232, 33, 65, 3333];
    for element in _a.iter() {
        println!("element = {}", element);
    }
    println!("----- end for -------");
    _a.len().try_into().unwrap()
}


fn while_fn(ii: u32) -> u32 {
    println!("----- start while -------");
    let mut repeat = 0;
    while repeat <= ii {
        println!("While loop {}", repeat);
        repeat += 1;
    }
    println!("----- end while -------");
    ii
}


fn loop_fn(ii: u32) -> u32 {
    println!("------ start loop ------");
    let mut i = 0;
    loop {
        println!("Еще раз");
        i += 1;
        if i > ii {
            println!("------ end loop ------");
            break ii;
        }
    }
}