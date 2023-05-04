fn main() -> Result<(), anyhow::Error> {

    let manifest_root = env!("CARGO_MANIFEST_DIR");
    let path = std::path::Path::new(manifest_root).join("the_history/RELEASES.md");
    let history = std::fs::read_to_string(path)?;


    let test = r"
FIRST SECTION!!
=====

para1 para para

p2 p2 p2 p2

Next heading
============

p3 p3 [p3][0202]

Third head
----------

third start

thirg end end

[0202]: http://foon.net/
[doop]: http://foon.net/

";

    let tree = markdown::to_mdast(&history, &markdown::ParseOptions::gfm()).expect("could not parse markdown");
    // println!("{:#?}", tree);
let kids = tree.children();
// kids = 3;
    let s = [1,2,3,4,0,1,2,3,0,5,6];
    let mut it = s.split(|n| *n==0);
    for sl in it {
        // println!("{:?}", sl);
    }

    // go through in reverse order collecting top l
    // get top level children
    // go through in reverse order collecting top level headings
    // output them in reverse

    let mut toplevel_flat = tree.children().unwrap().clone();
    toplevel_flat.reverse();
    let mut toplevel_sections = toplevel_flat.split_inclusive(|node| 
        match node {
            markdown::mdast::Node::Heading(markdown::mdast::Heading {depth: 1,..}) => true,
            _ => false,
        });

    let mut output = r#"<!DOCTYPE html>
<html lang="en-US">
<head>
<title>Rust Release Notes</title>
<meta charset="utf-8">
<style>
    h2,h3 {margin: 0;}
    html {background: #ddd;}
    li,p { margin: 0.3em; }
    .sticky {position: sticky;
        top: 0.3em;
        z-index: 3;
        margin: 0em;
    }
    .sticky-subheader {position: sticky;
        top: 2em;
        z-index: 2;
    }
    .solid-background {
        display: inline-block;
        background: #ddd;
        margin: 0.2em;
        padding-right: 0.4em;
    }

    .code {
        font-weight: bold;
        background-color: #ccc;
    }

    pre {font-size: 1em;}

    @media (prefers-color-scheme: dark) {
        html{background: #111;
            color: #bbb;
        }
        .solid-background {
            background: #111
        }
        .code {
            background-color: #333;
        }
        a {
            color: #68f;
        }
        a:visited {
            color: #d7f;
        }
    }
/**/
</style>
</head>
<body>
<p>Originael at: <a href="https://github.com/rust-lang/rust/blob/master/RELEASES.md">https://github.com/rust-lang/rust/blob/master/RELEASES.md</a>, there are at least 3 missing close-backticks towards the end of the file.
"#.to_owned();


    use std::io::Write;
    let mut file = std::fs::File::create(std::path::Path::new(manifest_root).join("the_history/rust_history.html"))?;
    file.write(output.as_ref())?;

    for section in toplevel_sections {
//        println!("\n\n\n{:#?}", section);
        let section = process_section(&section);
        file.write(section.as_ref())?;
    }
    // output.push_str("</body>");
    file.write("</body>".as_ref())?;

    Ok(())
}

use markdown::mdast;

fn process_section(nodes: &[mdast::Node]) -> String {
    let (defns, text): (Vec<_>, Vec<_>) = nodes.into_iter().partition(|node| if let markdown::mdast::Node::Definition(_) = node {true} else {false});

    let mut definitions = std::collections::HashMap::new();
    for entry in defns {
        if let mdast::Node::Definition(def) = entry {
            definitions.insert(def.label.as_ref().unwrap(), def.url.clone());
        }
    }

    let mut subsections = text.split_inclusive(|node| 
        match node {
            markdown::mdast::Node::Heading(markdown::mdast::Heading {depth: 2,..}) => true,
            _ => false,
        }).collect::<Vec<_>>();

    // subsections.reverse(); 
    // println!("section {:#?}", subsections);

    let mut acc = String::new();
    acc.push_str("<div><hr>");

    // // let mut it = 
    let mut start_of_section = true;
    // // make a section div for the whole thing
    // // println!("subs {:#?}", subsections[0]);
        let mut first_subsection = true;
    for subs in subsections.iter().rev() {
        for para in subs.iter().rev() {
            match para {
                mdast::Node::Heading(heading) =>
                    if start_of_section {
                        acc.push_str(&std::format!("\n<span class='sticky'><div class='solid-background'><h2>{}</h2></div></span>\n", child_nodes_to_html(&heading.children, &definitions)));
                        start_of_section = false;
                    } else {
                        if ! first_subsection {
                            acc.push_str("</div>");
                        }
                        first_subsection = false;
                        acc.push_str(&std::format!("\n<div><span class='sticky-subheader'><div class='solid-background'><h3>{}</h3></div></span>\n", child_nodes_to_html(&heading.children, &definitions)));
                        // start_of_subsection =  false;
                    },
                mdast::Node::Paragraph(paragraph) =>
                    acc.push_str(&std::format!("\n<p>{}</p>\n", child_nodes_to_html(&paragraph.children, &definitions))),
                mdast::Node::List(list) =>
                    acc.push_str(&std::format!("\n<ul>\n{}\n</ul>\n", child_nodes_to_html(&list.children, &definitions))),
                mdast::Node::Code(code) =>
                    acc.push_str(&format!("<pre>{}</pre>\n", &code.value)),
                u => println!("unimplemented para: {:#?}", u),
            }

            // println!("para: {:#?}", para);
        }
        // acc.push_str("</div>");
    }
    acc.push_str("</div>");

    // close section header

    // acc.push_str(subsections)
    // for i in sub

    acc
}
use std::collections::HashMap;
fn child_nodes_to_html(nodes: &[mdast::Node], definitions: &HashMap<&String,String>) -> String {
    let mut acc = String::new();
    for n in nodes {
        match n {
            mdast::Node::Text(t) =>
                acc.push_str(&t.value),
            mdast::Node::LinkReference(linkr) => {
                if let Some(url) = definitions.get(&linkr.identifier) {
                    acc.push_str(&format!("<a href='{}'>{}</a>", url, child_nodes_to_html(&linkr.children, &definitions)));
                } else {
                    acc.push_str(&format!("<a href='{}'>{}</a>", linkr.identifier, child_nodes_to_html(&linkr.children, &definitions)));
                }
            }
            mdast::Node::ListItem(li) =>
                acc.push_str(&format!("<li>{}</li>\n", child_nodes_to_html(&li.children, &definitions))),
            mdast::Node::Paragraph(p) =>
                acc.push_str(&format!("<p>{}</p>\n", child_nodes_to_html(&p.children, &definitions))),
            mdast::Node::Link(mdast::Link {children, url, ..}) =>
                acc.push_str(&format!("<a href='{}'>{}</a>", url, child_nodes_to_html(&children, &definitions))),
            mdast::Node::Html(html) =>
                acc.push_str(&html.value),
            mdast::Node::BlockQuote(bq) =>
                acc.push_str(&format!("<blockquote>{}</blockquote>\n", child_nodes_to_html(&bq.children, &definitions))),
            mdast::Node::Delete(del) =>
                acc.push_str(&format!("<span style='text-decoration: line-through;'>{}</span>\n", child_nodes_to_html(&del.children, &definitions))),
            mdast::Node::Emphasis(em) =>
                acc.push_str(&format!("<em>{}</em>\n", child_nodes_to_html(&em.children, &definitions))),
            mdast::Node::Strong(strong) =>
                acc.push_str(&format!("<b><i>{}</i></b>\n", child_nodes_to_html(&strong.children, &definitions))),
            mdast::Node::InlineCode(inline) =>
                acc.push_str(&format!("<span class='code'>{}</span>\n", &inline.value)),
            mdast::Node::Code(code) =>
                acc.push_str(&format!("<pre class='code'>{}</pre>\n", &code.value)),
            mdast::Node::List(list) =>
                acc.push_str(&std::format!("\n<ul>\n{}\n</ul>\n", child_nodes_to_html(&list.children, &definitions))),


            u => println!("unimplemented node: {:#?}", u),
        }
    }
    acc
}