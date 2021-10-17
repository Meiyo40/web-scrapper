#[derive(Debug)]
pub struct List {
    list: Vec<String>,
}

impl List {
    pub fn get_element(&self, idx: usize) -> &String {
        &self.list[idx]
    }

    pub fn init() -> List {
        List {
            list: vec![String::from("http://www.opex360.com/")],
        }
    }

    pub fn add_website(&mut self, website: String) {
        self.list.push(website);
    }
}
