pub mod core;
pub mod error;
pub mod utils;

pub fn radha() -> &'static str {
    "krsna"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let krsna = radha();
        println!("{krsna}");
        assert_eq!(krsna, "krsna")
    }
}
