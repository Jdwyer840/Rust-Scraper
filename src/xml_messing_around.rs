use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};


// #[derive(Debug, Serialize, Deserialize)]
// struct Function {
//     #[serde(rename = "name")]
//     function_name: String,
//     parameters: Vec<Parameter>,
//     body: String,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// struct Parameter {
//     #[serde(rename = "name")]
//     param_name: String,
//     #[serde(rename = "type")]
//     param_type: String,
// }

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct XMLTest {
    #[serde(rename = "data")]
    xml_data: Data,
}

pub fn test_xml_stuff() {
    let xml_data = r#"
    <xmlTest>

<data>
<url>google.com</url>
</data>
</xmlTest>
    "#;
    let xml_test: XMLTest = from_str(xml_data).unwrap();
    println!("Deserialized: {:?}", xml_test);

    let xml_output = to_string(&xml_test).unwrap();
    println!("Serialized: {}", xml_output);
}