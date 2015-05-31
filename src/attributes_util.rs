use html5ever::tokenizer::{Attribute};


pub trait AttributesUtil {

    fn get_value(&self, name: &str) -> Option<String>;

}

impl AttributesUtil for Vec<Attribute> {

    fn get_value(&self, name: &str) -> Option<String> {
        self.iter()
        .find(|attr| {
            attr.name.local.to_lowercase() == name
        })
        .map(|attr| {
            attr.value.clone()
        })
    }

}
