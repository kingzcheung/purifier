use json::{code_generator::CodeGenerator, parser::Parser, value::Json};
use pyo3::prelude::*;

pub mod json;

pub fn parse(s: &str) -> Json {
    let mut s1 = s;
    let mut js = Json::Null;
    for (index, char) in s.chars().enumerate() {
        if char == '{' {
            s1 = &s[index..];
            let mut parser = Parser::new(s1);
            let res = parser.parse();
            match res {
                Ok(json_raw) => {
                    js = json_raw;
                    break;
                }
                Err(_e) => {
                    // println!("{:?}",e);
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

    #[test]
    fn test_purifier_parse() {
        let s = r#"
        sdfs json```{ s {"name":"沙滩上的女子和狗","category":"宠物用品","packaging":"","tags":["海滩","户外活动"],"compositions":[],"description":"这张照片展示了一位女士在海滩上享受时光时与她的狗狗互动的情景。","shooting_angle":""}```ssdf
        "#;
        let res = super::parse(s);
        println!("{}", super::stringify(res));
    }
}
