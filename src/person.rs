pub struct User {
    name: String,
    age: i8,
    bio: String,
    email: String,
    is_active: bool,
    gender: char,
}

impl User {
    pub fn create(name: String, age: i8, bio: String, email: String, gender: char) -> Self {
        Self {
            name,
            age,
            bio,
            email,
            gender,
            is_active: false,
        }
    }

    pub fn introduce(&self) -> String {
        let posessive_pronoun: &str = if self.gender == 'M' { "His" } else { "Her" };
        format!(
            "{} is {} years old.\n {} email is {}.\n\n Bio: {}",
            self.name,
            self.age,
            posessive_pronoun,
            self.email,
            self.bio
        )
    }
}
