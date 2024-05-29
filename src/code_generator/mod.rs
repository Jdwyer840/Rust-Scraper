pub use crate::xml_parsing::types::*;
use crate::xml_parsing::types::WebScrape;


// this will be taking an object only

fn generate_rust_code(xml_test: &WebScrape, config: &Config) -> String {
    let mut code = String::new();

    // Example code for connecting to WebDriver
    code.push_str("let c = ClientBuilder::native().connect(\"http://localhost:4444\").await.expect(\"failed to connect to WebDriver\");\n");
    code.push_str("// Connecting using Rustls (with feature `rustls-tls`)\n");
    code.push_str("// let c = ClientBuilder::rustls().connect(\"http://localhost:4444\").await.expect(\"failed to connect to WebDriver\");\n\n");

    // Example code for first step
    code.push_str(&format!("c.goto(\"{}\").await?;\n", xml_test.data.url));

    for step in &xml_test.steps.step {
        if step.action == "goTO" {
            code.push_str(&format!("c.goto(\"{}\").await?;\n", step.url));
        }
    }

    code
}
