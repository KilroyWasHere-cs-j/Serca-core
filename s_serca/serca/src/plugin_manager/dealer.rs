use libloading::{ Library, Symbol };

pub struct Dealer {
    plugins: Vec<Library>
}
