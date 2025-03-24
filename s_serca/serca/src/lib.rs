use anyhow::Result;

pub mod web;
use crate::web::marionette;
use crate::web::puppeteer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
