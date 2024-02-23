pub fn radha() -> &'static str {
    "krsna"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let radha = radha();
        println!("{radha}");
        assert_eq!(radha, "krsna");
    }
}
