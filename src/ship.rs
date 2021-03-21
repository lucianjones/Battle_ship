// define struct ship, property n_spaces defines ship len
pub struct Ship {
    pub name: String,
    pub n_spaces: i8,
}
// returns first letter of ship name to use as board marker
impl Ship {
    pub fn get_label(&self) -> char {
        self.name.chars().nth(0).unwrap()
    }
}

// creates instance of ship struct
pub fn ship_factory(name: String, n_spaces: i8) -> Ship {
    return Ship {
        name: name,
        n_spaces: n_spaces,
    }
}
