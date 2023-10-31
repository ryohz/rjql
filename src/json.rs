use serde_json::Value;

use crate::error::RefError;

pub struct Json {
    pub data: serde_json::Value,
}

impl Json {
    pub fn new(data: &String) -> Self {
        Json {
            data: serde_json::from_str::<Value>(data).unwrap(),
        }
    }

    pub fn refer(&mut self, query: &String) -> Result<String, RefError> {
        let v = self.refer_(query)?;
        Ok(v.to_string())
    }

    fn refer_(&mut self, query: &String) -> Result<&mut Value, RefError> {
        let mut current: &mut Value = &mut self.data;
        if query.is_empty() {
            return Err(RefError::EmptyQuery);
        }
        if start_or_end_by_dot(&query) {
            return Err(RefError::StartOrEndByDot);
        }
        let query = query.split('.').collect::<Vec<&str>>();
        for token in query {
            if is_contain_list_ref(token) {
                if unclosed_bracket(&token) {
                    return Err(RefError::UnclosedBracket);
                }
                if not_num_in_bracket(&token) {
                    return Err(RefError::NotNumInBracket);
                }
                if not_ends_with_bracket(&token) {
                    return Err(RefError::NotEndWithBracket);
                }
                let iter = token.split('[').collect::<Vec<&str>>();
                let name = iter[0];
                let index = iter[1].split(']').collect::<Vec<&str>>()[0]
                    .to_string()
                    .parse::<usize>()
                    .unwrap();
                let next = &mut current[name][index];
                current = next;
            } else {
                let r = current.get(token);
                if r.is_none() {
                    return Err(RefError::NotFound);
                }
                current = &mut current[token];
            }
        }
        Ok(current)
    }
}

fn start_or_end_by_dot(query: &String) -> bool {
    let start = query.get(0..1).unwrap();
    let end = query.get(query.len() - 1..query.len()).unwrap();
    start == "." || end == "."
}

fn not_num_in_bracket(token: &str) -> bool {
    let c = token.split('[').collect::<Vec<&str>>()[1]
        .split(']')
        .collect::<Vec<&str>>()[0];
    let r = c.parse::<usize>();
    r.is_err()
}

fn not_ends_with_bracket(token: &str) -> bool {
    "]" != token.get(token.len() - 1..token.len()).unwrap()
}

fn unclosed_bracket(token: &str) -> bool {
    !token.contains(']')
}

fn is_contain_list_ref(token: &str) -> bool {
    return token.contains('[');
}
