use pulldown_cmark::{Parser, Options, html};

fn main() {
    let md = "Here is **bold** and *italic* and inline math $a^2$ and block math $$b^2$$.";
    let mut options = Options::empty();
    options.insert(Options::ENABLE_MATH);
    let parser = Parser::new_ext(md, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    println!("{}", html_output);
}
