#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

use anyhow::Result;
use mdbook::book::Book;
use mdbook::book::BookItem;
use mdbook::errors::Error;
use mdbook::preprocess::Preprocessor;
use mdbook::preprocess::PreprocessorContext;
use pulldown_cmark::{Event, Options, Parser, Tag};
use pulldown_cmark_to_cmark::fmt::cmark;
use std::io::Write;
use std::process::{Command, Stdio};

pub enum RenderMode {
    ASCII,
    SVG,
}

pub struct CodeProcessor;

impl CodeProcessor {
    pub fn new() -> CodeProcessor {
        CodeProcessor
    }
}

impl Preprocessor for CodeProcessor {
    fn name(&self) -> &str {
        "nop-preprocessor"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let cfg = get_kubook_config(ctx);
        debug!("{:?}", cfg);
        book.for_each_mut(|item: &mut BookItem| {
            if let BookItem::Chapter(ref mut chapter) = *item {
                chapter.content = render_md_code_blocks(
                    &chapter.content,
                    "plantumltxt",
                    &RenderMode::ASCII,
                    &cfg,
                    &render_plantuml,
                );

                chapter.content = render_md_code_blocks(
                    &chapter.content,
                    "plantumlsvg",
                    &RenderMode::SVG,
                    &cfg,
                    &render_plantuml,
                );
            }
        });
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}

pub fn render_md_code_blocks(
    markdown: &str,
    code_id: &str,
    output_type: &RenderMode,
    cfg: &KuBookConfig,
    render_code_block: &dyn Fn(&String, &KuBookConfig, &RenderMode) -> String,
) -> String {
    let options = Options::all();
    let parser = Parser::new_ext(markdown, options);

    let mut in_code_block = false;
    let mut code_source = String::from("");

    let events = parser.map(|event| match event {
        Event::Start(Tag::CodeBlock(code)) => {
            if code.clone().into_string() == code_id {
                debug!("Started {} code block", code_id);
                in_code_block = true;
                Event::Text("".into())
            } else {
                Event::Start(Tag::CodeBlock(code))
            }
        }
        Event::Text(text) => {
            if in_code_block {
                code_source.push_str(&text.into_string());
                Event::Text("".into())
            } else {
                Event::Text(text)
            }
        }
        Event::End(Tag::CodeBlock(code)) => {
            if code.clone().into_string() == code_id {
                in_code_block = false;
                let plantuml_code = render_code_block(&code_source.clone(), &cfg, output_type);
                code_source = String::from("");

                Event::Text(plantuml_code.clone().into())
            } else {
                Event::End(Tag::CodeBlock(code))
            }
        }
        _ => event,
    });

    let mut markdown = String::with_capacity(markdown.len() + 1024);
    cmark(events, &mut markdown, None).unwrap();

    markdown
}

//
//  plantuml_cli_output(
//    "Bob -> Alice : hello\nAlice->Bob: yes\n",
//    "/usr/bin/java",
//    "/home/xxx/yyy/zzz/plantuml.jar",
//    "-ttxt"
//  );

pub fn plantuml_cli(
    plantuml_code: &str,
    java_path: &str,
    plantuml_jar_path: &str,
    output_type_arg: &str,
) -> Result<String> {
    // sh -c "/usr/bin/java -jar /home/xxx/yyy/zzz/plantuml.jar -ttxt -pipe"
    let args = [
        java_path,
        "-jar",
        plantuml_jar_path,
        output_type_arg,
        "-pipe",
    ];
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(args.join(" "))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    {
        let child_stdin = child.stdin.as_mut().unwrap();
        child_stdin.write_all(plantuml_code.as_bytes())?;
    }

    let output = child.wait_with_output()?;
    let result = format!("{}", String::from_utf8(output.stdout).unwrap());
    Ok(result)
}

pub fn render_plantuml(
    plantuml_code: &String,
    cfg: &KuBookConfig,
    output_type: &RenderMode,
) -> String {
    let java = cfg.java_path.as_ref().unwrap();
    let jar = cfg.plantuml_jar.as_ref().unwrap();
    match output_type {
        RenderMode::ASCII => {
            let cli_output = plantuml_cli(plantuml_code, java.as_str(), jar.as_str(), "-ttxt");
            // "## You are hello ?\n\n".to_string()
            format!("\n```\n{}\n```\n\n", cli_output.unwrap())
        }
        RenderMode::SVG => {
            let cli_output = plantuml_cli(plantuml_code, java.as_str(), jar.as_str(), "-tsvg");
            // "## You are hello ?\n\n".to_string()
            // <img src='data:image/svg+xml;utf8,<svg ... > ... </svg>'>
            // <img alt="" src="data:image/svg+xml;base64,PHN2ZyB">
            // ![alt](data:image/svg+xml;base64,PHN2ZyB4bWxucz0i)
            let base64_encode_svg = base64::encode(&cli_output.unwrap());
            //format!("\n<p><img alt=\"\" src=\"data:image/svg+xml;base64,{}\"/>\n</p>\n", base64_encode_svg)
            format!(
                "\n![alt](data:image/svg+xml;base64,{})\n",
                base64_encode_svg
            )
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct KuBookConfig {
    pub command: Option<String>,
    pub java_path: Option<String>,
    pub plantuml_jar: Option<String>,
}

impl Default for KuBookConfig {
    fn default() -> KuBookConfig {
        KuBookConfig {
            command: None::<String>,
            java_path: None::<String>,
            plantuml_jar: None::<String>,
        }
    }
}

fn get_kubook_config(ctx: &PreprocessorContext) -> KuBookConfig {
    debug!("{:?}", ctx);
    debug!("{:?}", ctx.config.get("preprocessor.kubook"));
    match ctx.config.get("preprocessor.kubook") {
        Some(raw) => raw
            .clone()
            .try_into()
            .or_else(|e| {
                warn!(
                    "Failed to get config from book.toml, using default configuration ({}).",
                    e
                );
                Err(e)
            })
            .unwrap_or(KuBookConfig::default()),
        None => KuBookConfig::default(),
    }
}
