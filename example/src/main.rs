extern crate scrap_util;

use scrap_util::hyper::Client;
use scrap_util::decode::decode;
use scrap_util::{RcDom, parse, one_input, Element};
use scrap_util::dom_util::DomUtil;
use scrap_util::attributes_util::AttributesUtil;

use std::io::Read;
use std::default::Default;

macro_rules! ok_or_return {
    ($expression: expr, $message: expr) => (
        match $expression {
            Ok(result) => result,
            Err(err) => {
                println!("Error: {}: {}", $message, err);
                return;
            }
        }
    )
}

macro_rules! some_or_return {
    ($expression: expr, $message: expr) => (
        match $expression {
            Some(result) => result,
            None => {
                println!("{}", $message);
                return;
            }
        }
    )
}


fn main() {

    let url = "http://www.rust-lang.org/";
    let mut client = Client::new();
    let mut response = ok_or_return!(client.get(url).send(), "Cannot send request");

    let mut bytes = Vec::new();
    ok_or_return!(response.read_to_end(&mut bytes), "Cannot read response");

    let markup = ok_or_return!(decode(&response, &bytes), "Cannot decode body");

    let dom: RcDom = parse(one_input(markup), Default::default());

    // Get the first UL element with the class name "row menu"
    let menu = some_or_return!(dom.document.find_element(|name, attrs| {
        name.local.to_string() == "ul" && attrs.get_value("class") == Some("row menu".to_string())
    }), "Menu element not found");

    // For each list item of the menu
    for child in menu.borrow().children.iter() {
        // Get the first H2 element
        match child.find_element(|name, _| name.local.to_string() == "h2") {
            Some(title) => {
                // If it is found, print its content
                println!("{}", title.get_text());
                // And print links from the list item
                for link in child.find_elements(|name, _| name.local.to_string() == "a").iter() {
                    match link.borrow().node {
                        Element(_, ref attrs) => println!("* {}: {}", link.get_text(), attrs.get_value("href").unwrap()),
                        _ => {},
                    }
                }
                println!("");
            },
            _ => {},
        }
    }

}
