pub struct Department {
    value: String,
}

impl Department {
    pub fn new(value: String) -> Department {
        let mut departments: Vec<String> = Vec::new();

        departments.push(String::from("Engineering"));
        departments.push(String::from("Sales"));
        departments.push(String::from("Purpose"));

        if departments.contains(&value) {
            Department { value }
        } else {
            panic!("Invalid department!");
        }
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn print_valid_departments() {
        let mut departments: Vec<String> = Vec::new();

        departments.push(String::from("Engineering"));
        departments.push(String::from("Sales"));
        departments.push(String::from("Purpose"));

        println!("Valid departments are: {departments:?}");
    }
}
