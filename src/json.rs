use std::str::FromStr;

use serde_json::Value;

use crate::error::Error;

enum RefPurpose {
    Ref,
    Mod,
}

pub struct Json {
    pub data: serde_json::Value,
}

impl Json {
    pub fn new(data: &String) -> Self {
        Json {
            data: serde_json::from_str::<Value>(data).unwrap(),
        }
    }

    pub fn refer(&mut self, query: &String) -> Result<String, Error> {
        let v = self.refer_(query, RefPurpose::Ref)?;
        match v {
            Value::String(v) => Ok(v.to_string()),
            _ => Ok(v.to_string()),
        }
    }

    pub fn delete(&mut self, path: &String) -> Result<(), Error> {
        let locate_vec = path.split('.').collect::<Vec<&str>>();

        let last_locate = locate_vec.get(locate_vec.len() - 1).unwrap();
        // {
        //      "aaa": {
        //          *locate*
        //          "bbb": "ccc",
        //          "ddd": "eee"
        //      }
        // {
        // prepare a mutable reference of parent of target, update parent with copy of parent that doesn't contain target
        // with above example(target is bbb), prepare the mutable reference of aaa, delete bbb from copy of aaa, update aaa with copy of it
        // if that target of deletion is an element of an array, parent element is array that contains target
        // 指定された要素の親要素の可変参照をrefer_を使って用意し、親要素のコピーの中から削除対象の要素を削除したオブジェクトで親要素を更新する
        // 上の図でいうとキー名bbbが削除対象である場合、aaaの値を取得して、そのなかからキー名がbbbの要素を削除したものでaaaを更新する
        // 削除対象が配列の一要素である場合は、親要素は対象を含んでいる配列全体になる
        //
        // 削除対象が配列の要素かどうかを、パスの最後の部分(aaa.bbb.ccc[1]ならccc[1])が"["と"]"を含んでいるかどうかで判定している
        // したがって、要素の名前に大括弧を含むことはできない。
        // determining whether a target of deletion is an element of array with ...
        // knowing whether the last part of location (i.e aaa.bbb.ccc[1] -> ccc[1]) contains both [ and ],
        // both [ and ] can't be contained by name of element
        if last_locate.contains(&"[") && last_locate.contains(&"]") {
            let token = last_locate;
            if unclosed_bracket(token) {
                return Err(Error::UnclosedBracket);
            }
            if not_num_in_bracket(token) {
                return Err(Error::NotNumInBracket);
            }
            if not_ends_with_bracket(token) {
                return Err(Error::NotEndWithBracket);
            }
            let iter = token.split('[').collect::<Vec<&str>>();
            let name = iter[0];
            let index = iter[1].split(']').collect::<Vec<&str>>()[0]
                .to_string()
                .parse::<usize>()
                .unwrap();
            let mut parent_locate = locate_vec.get(0..locate_vec.len() - 1).unwrap().to_vec();
            parent_locate.push(name);
            let parent_locate = parent_locate.join(".");
            let parent = self.refer_(&parent_locate, RefPurpose::Mod)?;
            match parent.clone() {
                Value::Array(arr) => {
                    let mut new = arr;
                    new.remove(index);
                    *parent = Value::Array(new);
                }
                _ => (),
            }
            Ok(())
        } else {
            let parent_locate = locate_vec.get(0..locate_vec.len() - 1).unwrap().join(".");
            let dest_name = locate_vec
                .get(locate_vec.len() - 1..locate_vec.len())
                .unwrap()
                .join("");
            let parent = self.refer_(&parent_locate, RefPurpose::Mod)?;
            if let Value::Object(map) = parent.clone() {
                let mut new_map = serde_json::Map::new();
                for (k, v) in map {
                    if k == dest_name {
                        continue;
                    }
                    new_map.insert(k, v);
                }
                let new_child = Value::from(new_map);
                *parent = new_child;
            }
            Ok(())
        }
    }

    pub fn modify(&mut self, locate: &String, raw_value: &String) -> Result<(), Error> {
        let value = to_value(raw_value);
        let dest = self.refer_(locate, RefPurpose::Mod)?;
        *dest = value;
        Ok(())
    }

    // function return a mutable reference of element of specified path
    // if the purpose of the reference is reading, return NotFound error when no element that specified path is pointing exists
    // if if's modifying, making new element that is initialized by null, update data, and return imutable reference of new null element
    // 指定されたパスの可変参照を返す関数
    // 参照の目的が単に閲覧である場合は、存在しない要素を参照しようとしたときにNotFoundを返す
    // 参照の目的が改変だった場合は、存在しない要素をnullで初期化してdataに代入して、その新しくできたnullの要素の可変参照を返す
    fn refer_(&mut self, path: &String, ref_purpose: RefPurpose) -> Result<&mut Value, Error> {
        let mut current = &mut self.data;
        if path.to_string() == ".".to_string() {
            return Ok(current);
        }
        if path.is_empty() {
            return Err(Error::EmptyQuery);
        }
        if start_or_end_by_dot(&path) {
            return Err(Error::StartOrEndByDot);
        }
        let query = path.split('.').collect::<Vec<&str>>();
        for token in query {
            if is_contain_list_ref(token) {
                if unclosed_bracket(&token) {
                    return Err(Error::UnclosedBracket);
                }
                if not_num_in_bracket(&token) {
                    return Err(Error::NotNumInBracket);
                }
                if not_ends_with_bracket(&token) {
                    return Err(Error::NotEndWithBracket);
                }
                let iter = token.split('[').collect::<Vec<&str>>();
                let name = iter[0];
                let index = iter[1].split(']').collect::<Vec<&str>>()[0]
                    .to_string()
                    .parse::<usize>()
                    .unwrap();
                if current.get(name).is_none() {
                    match ref_purpose {
                        RefPurpose::Ref => {
                            return Err(Error::NotFound);
                        }
                        RefPurpose::Mod => {
                            current[name] = Value::Null;
                        }
                    }
                }
                let next = current.get_mut(name).unwrap().get_mut(index).unwrap();
                current = next;
            } else {
                if current.get(token).is_none() {
                    match ref_purpose {
                        RefPurpose::Ref => {
                            return Err(Error::NotFound);
                        }
                        RefPurpose::Mod => {
                            current[token] = Value::Null;
                        }
                    }
                }
                current = current.get_mut(token).unwrap();
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

fn to_value(raw: &String) -> Value {
    // empty
    if raw.is_empty() {
        return Value::String("".to_string());
    }
    // number
    let num_r = serde_json::Number::from_str(raw.as_str());
    if num_r.is_ok() {
        return Value::Number(num_r.unwrap());
    }
    // bool
    if raw == "true" {
        return Value::Bool(true);
    }
    if raw == "false" {
        return Value::Bool(false);
    }
    // null
    if raw == "null" {
        return Value::Null;
    }
    // array
    let start = raw.get(0..1).unwrap();
    let end = raw.get(raw.len() - 1..raw.len()).unwrap();
    if start == "[" && end == "]" {
        let mut vec_value = Vec::<Value>::new();
        let contents_raw = raw.get(1..raw.len() - 1).unwrap();
        let contents = contents_raw.split(',').collect::<Vec<&str>>();
        for content in contents {
            vec_value.push(to_value(&content.to_string()));
        }
        return Value::Array(vec_value);
    }
    // String
    return Value::String(raw.to_string());
}
