pub struct Foundry {
    test_text: &'static str,
}

impl Foundry {
    pub fn new() -> Foundry {
        Foundry {
            test_text: "I am an ide, i promise!",
        }
    }
}
