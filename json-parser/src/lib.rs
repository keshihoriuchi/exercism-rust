// use std::collections::HashMap;
use std::collections::BTreeMap;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<BTreeMap<String, Json>>),
}

impl Json {
    fn string(self) -> String {
        if let Json::String(s) = self {
            s
        } else {
            panic!("Not a String")
        }
    }
}

peg::parser!( grammar json() for str {

    // Base
    pub rule value() -> Json
        = boolean_true() / boolean_false() / null() / string() / object() / array() / number()
    
    rule _()
        = [' ' | '\t' | '\n' | '\r']*

    rule boolean_true() -> Json
        = _ "true" _ { Json::Boolean(true) }

    rule boolean_false() -> Json
        = _ "false" _ { Json::Boolean(false) }

    rule null() -> Json
        = _ "null" _ { Json::Null }


    // Object
    rule object() -> Json
        = _ "{" _ members:member() ** "," _"}" _ {
            let mut h = BTreeMap::new();
            for m in members {
                h.insert(m.0, m.1);
            }
            Json::Object(Box::new(h))
        } 
    
    rule member() -> (String, Json)
        = _ name:string() _ ":" _ value:value() _ {
            (name.string(), value)
        }


    // Array
    rule array() -> Json
        = _ "[" v:array_value() ** "," _ "]" _ {
            Json::Array(v)
        }

    rule array_value() -> Json
        = _ v:value() _ { v }


    // Number
    rule number() -> Json
        = result:$(("-")? ("0" / ['1'..='9'] ['0'..='9']*) ("." ['0'..='9']+)? (['e' | 'E'] (['-' | '+'])? ['0'..='9']+)?) {
            Json::Number(result.parse().unwrap())
        }


    // String
    rule string() -> Json
        = _ "\"" v:(unescaped() / escaped())* "\"" _ { 
            let v: String = v.into_iter().collect();
            Json::String(v)
        }

    rule unescaped() -> char
        = c:$(['\u{0020}' | '\u{0021}' | '\u{0023}'..='\u{005B}' | '\u{005D}'..=std::char::MAX]) {
            c.chars().next().unwrap()
        }

    rule escaped() -> char
        = "\\\\" { '\\' } / 
          "\\\"" { '"' } /
          "\\/" { '/' } /
          "\\b" { '\u{0008}' } /
          "\\f" { '\u{000c}' } /
          "\\n" { '\n' } /
          "\\r" { '\r' } /
          "\\t" { '\t' } /
          "\\u" digits:$(['0'..='9' | 'a'..='f']*<4>) { u8::from_str(digits).unwrap() as char }
});

#[cfg(test)]
mod tests {
    use super::json;
    use super::Json;
    use std::collections::BTreeMap;
    #[test]
    fn json_works() {
        assert_eq!(json::value("true"), Ok(Json::Boolean(true)));
        assert_eq!(json::value("false"), Ok(Json::Boolean(false)));
        assert_eq!(json::value("null"), Ok(Json::Null));

        assert_eq!(
            json::value("\"a bc\""),
            Ok(Json::String(String::from("a bc")))
        );
        assert_eq!(
            json::value("\"\\nab\\\"\""),
            Ok(Json::String(String::from("\nab\"")))
        );

        assert_eq!(
            json::value("{ }"),
            Ok(Json::Object(Box::new(BTreeMap::new())))
        );
        {
            let mut m = BTreeMap::new();
            m.insert(String::from("a"), Json::Boolean(true));
            assert_eq!(json::value("{\"a\": true}"), Ok(Json::Object(Box::new(m))));
        }
        {
            let mut m = BTreeMap::new();
            let mut m1 = BTreeMap::new();
            m1.insert(String::from("b"), Json::String(String::from("foo")));
            m.insert(String::from("a"), Json::Boolean(true));
            m.insert(String::from("c"), Json::Object(Box::new(m1)));
            assert_eq!(
                json::value("{  \"a\"  : true  , \"c\": {\"b\": \"foo\"}  }"),
                Ok(Json::Object(Box::new(m)))
            );
        }
        assert_eq!(json::value("[ ]"), Ok(Json::Array(vec![])));
        assert_eq!(
            json::value("[  \"a\"  , true  ]"),
            Ok(Json::Array(vec![
                Json::String(String::from("a")),
                Json::Boolean(true)
            ]))
        );
        assert_eq!(json::value("0"), Ok(Json::Number(0.0)));
        assert_eq!(json::value("1203"), Ok(Json::Number(1203.0)));
        assert_eq!(json::value("-1203"), Ok(Json::Number(-1203.0)));
    }
}
