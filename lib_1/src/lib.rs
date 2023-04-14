pub struct Munchy {
    name:String
}

impl Munchy {
    pub fn new(name: &str) -> Munchy {
        Munchy { name: name.to_string() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let m = Munchy::new("Munchy");
        assert_eq!(m.name(), "Munchy");
    }
}
