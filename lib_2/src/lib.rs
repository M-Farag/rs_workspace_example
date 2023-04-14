pub struct Mina {
    name:String
}

impl Mina {
    pub fn new(name: &str) -> Mina {
        Mina { name: name.to_string() }
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
        let m = Mina::new("Mina");
        assert_eq!(m.name(), "Mina");
    }
}