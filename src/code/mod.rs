pub struct Code {
    code: Vec<String>,
}

impl Default for Code {
    fn default() -> Code {
        Code {
            code: Vec::new(),
        }
    }
}

impl Code {
    pub fn new() -> Code {
        Code::default()
    }

    pub fn get(&self) -> String {
        self.code.join("")
    }

    // fn set(&mut self, code: Vec<String>) {
    //     self.code = code;
    // }

    pub fn add(&mut self, code: String) {
        self.code.push(code);
    }

    pub fn clear(&mut self) {
        self.code.clear();
    }
}