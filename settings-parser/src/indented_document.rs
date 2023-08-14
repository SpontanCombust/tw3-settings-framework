pub struct IndentedDocument {
    pub text: String,
    current_indent: u8  
}

impl IndentedDocument {
    pub fn new() -> Self {
        IndentedDocument { 
            text: String::new(),
            current_indent: 0 
        }
    }

    pub fn push_line(&mut self, s: &str) -> &mut Self {
        self.text.push_str("\t".repeat(self.current_indent.into()).as_str());
        self.text.push_str(s);
        self.new_line();
        self
    }

    pub fn new_line(&mut self) -> &mut Self{
        self.text.push_str("\n");
        self
    }

    // pub fn new_lines(&mut self, how_many: u8) -> &mut Self {
    //     self.text.push_str("\n".repeat(how_many.into()).as_str());
    //     self
    // }

    pub fn push_indent(&mut self) -> &mut Self {
        self.current_indent += 1;
        self
    }

    pub fn pop_indent(&mut self) -> &mut Self {
        self.current_indent -= 1;
        self
    }
}