use std::borrow::Cow;
use html5ever::tokenizer::{TokenSink, Token, TagKind, states, Tag};
use html5ever::driver::{tokenize_to, one_input};
use encoding::{Encoding, DecoderTrap};
use encoding::all::{ASCII, UTF_8};
use encoding::label::encoding_from_whatwg_label;

use hyper_crate::client::response::{Response};
use hyper_crate::header::{ContentType};
use hyper_crate::mime::{Mime, Value, Attr};
use hyper_crate::header::{Headers};

use attributes_util::AttributesUtil;

fn get_encoding_from_headers(headers: &Headers) -> Option<String> {
    match headers.get() {
        Some(&ContentType(Mime(_, _, ref params))) =>
            params.iter()
            .find(|&&(ref attr, _)| {
                match attr {
                    &Attr::Charset => true,
                    _ => false,
                }
            })
            .map(|&(_, ref value)| {
                match value {
                    &Value::Utf8 => "utf8".to_string(),
                    &Value::Ext(ref res) => res.clone(),
                }
            }),
        _ => None,
    }
}

struct Sink<'a> {
    stop: bool,
    headers: &'a mut Headers,
}

impl <'a> Sink<'a> {

    fn handle_meta(&mut self, tag: &Tag) {
        match tag.kind {
            TagKind::StartTag => {
                match (tag.attrs.get_value("http-equiv"), tag.attrs.get_value("content")) {
                    (Some(name), Some(value)) => {
                        self.headers.set_raw(name, vec![value.into_bytes()]);
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }

    fn handle_head(&mut self, tag: &Tag) {
        match tag.kind {
            TagKind::EndTag => self.stop = true,
            _ => {},
        }
    }

}

impl <'a>TokenSink for Sink<'a> {

    fn process_token(&mut self, token: Token) {
        match token {
            Token::TagToken(ref tag) => {
                match tag.name.as_slice() {
                    "meta" => self.handle_meta(tag),
                    "head" => self.handle_head(tag),
                    _ => {},
                }
            },
            _ => {},
        }
    }

    fn query_state_change(&mut self) -> Option<states::State> {
        if self.stop {
            Some(states::State::Quiescent)
        } else {
            None
        }
    }

}


fn extract_headers_from_html(body: &Vec<u8>) -> Headers {

    let mut headers = Headers::new();

    match ASCII.decode(body, DecoderTrap::Ignore) {
        Ok(markup) => {
            let sink = Sink {
                stop: false,
                headers: &mut headers,
            };

            tokenize_to(sink, one_input(markup), Default::default());
        },
        _ => {}
    }

    headers
}

pub fn decode(response: &Response, body: &Vec<u8>) -> Result<String, Cow<'static, str>> {
    let encoding = get_encoding_from_headers(&response.headers)
    .or_else(|| {
        get_encoding_from_headers(&extract_headers_from_html(body))
    })
    .unwrap_or("utf8".to_string());

    encoding_from_whatwg_label(encoding.as_str())
    .unwrap_or(UTF_8)
    .decode(body, DecoderTrap::Replace)
}
