#[derive(Clone, Debug)]
pub struct User {
    pub name: String,
    pub pass: String,
}

impl User {
    pub fn change_name(mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn change_pass(mut self, new_pass: String) {
        self.pass = new_pass;
    }

    pub fn print_all(&self) {
        println!("{}\n{}", self.name, self.pass);
    }

    pub fn is_pass_safe(self) -> bool {
        let mut safety = 0;

        for x in self.pass.chars() {
            if x == ' ' {
                continue;
            }

            if x.is_numeric() {
                safety += 1;
            }
            if x.is_ascii_punctuation() {
                safety += 1;
            }

            if safety >= 2 {
                return true;
            }
        }

        return false;
    }
}
