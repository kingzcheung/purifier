use json::{code_generator::CodeGenerator, parser::Parser, value::Json};
use pyo3::prelude::*;

mod data;
pub mod json;

pub fn parse(s: &str) -> Json {
    let mut js = Json::Null;
    // s.chars().enumerate() 得到的索引不是正确的索引
    for (index, char) in s.char_indices() {
        if char == '{' {
            let s1 = &s[index..];
            let mut parser = Parser::new(s1);
            let res = parser.parse();
            match res {
                Ok(json_raw) => {
                    if json_raw.is_null() {
                        continue;
                    }
                    if json_raw.is_object() {
                        if stringify(json_raw.clone()) == "{}" {
                            continue;
                        }
                    } else {
                        continue;
                    }
                    js = json_raw;
                    break;
                }
                Err(e) => {
                    println!("{:?}",e);
                    continue;
                }
            }
        }
    }

    js
}

pub fn stringify<T>(o: T) -> String
where
    T: Into<Json>,
{
    let mut gen = CodeGenerator::new();
    gen.gather(&o.into());
    gen.product()
}

#[pyfunction]
pub fn find(s: &str) -> PyResult<String> {
    Ok(stringify(parse(s)))
}

#[pymodule]
fn purifier(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(find, m)?)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{data::{self, data}, json::value::Json};
    use serde_json::{Result, Value};

    #[test]
    fn test_purifier_parse() {
        for (left, right) in data::data {
            let res = super::parse(left);
           
            if res == Json::Null {
                println!("{:?}", left);
            }
            assert!(res != Json::Null);
            let d = super::stringify(res);
            let v = serde_json::from_str::<Value>(&d);
            assert!(v.is_ok())
        }
    }
    #[test]
    fn test_single_purifier_parse(){
        let (left,right) = data::data[14];
        let res = super::parse(left);
        let d = super::stringify(res);
        assert!(!d.is_empty())
    }
}
