
use html5ever_dom_sink::rcdom::{Handle};
use html5ever_dom_sink::common::{Element, Text};
use html5ever::tokenizer::{Attribute};

use string_cache::QualName;


pub trait DomUtil {

    fn _get_text_acc(&self, &mut String);

    fn _find_one<P>(&self, &mut P) -> Option<Handle>
        where P: FnMut(&Handle) -> bool;

    fn _find_acc<P>(&self, &mut P, &mut Vec<Handle>)
        where P: FnMut(&Handle) -> bool;

    fn get_text(&self) -> String {
        let mut result = String::new();
        self._get_text_acc(&mut result);
        result
    }

    fn find_one<P>(&self, mut predicate: P) -> Option<Handle>
        where P: FnMut(&Handle) -> bool
    {
        self._find_one(&mut predicate)
    }

    fn find_element<P>(&self, mut predicate: P) -> Option<Handle>
        where P: FnMut(&QualName, &Vec<Attribute>) -> bool
    {
        self.find_one(|handle| {
            match handle.borrow().node {
                Element(ref name, ref attrs) => predicate(name, attrs),
                _ => false,
            }
        })
    }

    fn find<P>(&self, mut predicate: P) -> Vec<Handle>
        where P: FnMut(&Handle) -> bool
    {
        let mut result: Vec<Handle> = Vec::new();
        self._find_acc(&mut predicate, &mut result);
        result
    }

    fn find_elements<P>(&self, mut predicate: P) -> Vec<Handle>
        where P: FnMut(&QualName, &Vec<Attribute>) -> bool
    {
        self.find(|handle| {
            match handle.borrow().node {
                Element(ref name, ref attrs) => predicate(name, attrs),
                _ => false,
            }
        })
    }

}

impl DomUtil for Handle {

    fn _get_text_acc(&self, acc: &mut String) {
        match self.borrow().node {
            Text(ref text) => acc.push_str(text.as_str()),
            _ => self.borrow().children._get_text_acc(acc),
        }
    }

    fn _find_one<P>(&self, predicate: &mut P) -> Option<Handle>
        where P: FnMut(&Handle) -> bool
    {
        self.borrow().children._find_one(predicate)
    }

    fn _find_acc<P>(&self, predicate: &mut P, acc: &mut Vec<Handle>)
        where P: FnMut(&Handle) -> bool
    {
        self.borrow().children._find_acc(predicate, acc);
    }

}

impl DomUtil for Vec<Handle> {

    fn _get_text_acc(&self, acc: &mut String) {
        for handle in self.iter() {
            handle._get_text_acc(acc);
        }
    }

    fn _find_one<P>(&self, predicate: &mut P) -> Option<Handle>
        where P: FnMut(&Handle) -> bool
    {
        for handle in self.iter() {
            if predicate(handle) {
                return Some(handle.clone());
            }

            match handle._find_one(predicate) {
                Some(r) => return Some(r),
                _ => {}
            }
        }
        None
    }

    fn _find_acc<P>(&self, predicate: &mut P, acc: &mut Vec<Handle>)
        where P: FnMut(&Handle) -> bool
    {
        for handle in self.iter() {
            if predicate(handle) {
                acc.push(handle.clone());
            }

            handle._find_acc(predicate, acc);
        }
    }

}
