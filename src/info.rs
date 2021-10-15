#[derive(Clone, Debug)]
pub struct Info {
    bin_name: String,
    name: String,
    version: String,
}

impl Info {
    pub fn new() -> Self {
        Self {
            bin_name: String::from("ndsc"),
            name: String::from("N dimensional symmetric cipher"),
            version: String::from("v0.1.0"),
        }
    }

    pub fn bin_name(&self) -> &String {
        &self.bin_name
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn version(&self) -> &String {
        &self.version
    }
}
