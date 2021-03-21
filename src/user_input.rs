use text_io::try_read;

// define two functions which get the user input

pub fn get_y() -> i8 {  //returns i8 as all other funcs use i8/ val only 0-9
    let mut y: char = match try_read!() { //grabs char user inputed or recursively
        Ok(t) => { t },                   //calls itself till valid input
        Err(_e) => { 
            println!("That input is invalid, please try again");
            get_y();
            'x'
        },
    };
    y.make_ascii_uppercase();  // makes uppercase so input can be case insensitive
    let y = match y { // matches char to index value of row
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
        'I' => 8,
        'J' => 9,
        _ => { 
            println!("That input is invalid, please try again");
            get_y()  
        },
    };
    y as i8 //return row val
}

pub fn get_x() -> i8 { // same as top, but as we recieve a num just have to check -1 < x < 10
    let x = match try_read!() {
        Ok(t) => { t },
        Err(_e) => { 
            println!("That input is invalid, please try again");
            get_x()
        },
    };
    if (x < 0) || (x > 9) {
        println!("That input is invalid, please try again");
        get_x()
    } else { x as i8 } // return col val
}


