use text_io::try_read;

pub fn get_x() -> i8 {
    let x = match try_read!() {
        Ok(T) => { T },
        Err(_e) => { 
            println!("That input is invalid, please try again");
            get_x()
        },
    };
    if (x < 0) || (x > 9) {
                println!("That input is invalid, please try again");
                get_x()
    } else { x as i8 }
}

pub fn get_y() -> i8 {
    let y = match try_read!() {
        Ok(T) => { T },
        Err(_e) => { 
            println!("That input is invalid, please try again");
            get_y()
        },
    };
    if (y < 0) || (y > 9) {
        println!("That input is invalid, please try again");
        get_y()
    } else { y as i8 }
}

