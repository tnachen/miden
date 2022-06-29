use crate::ModuleMap;

use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::Path;

// CONSTANTS
// ================================================================================================

pub const COMMENT_PREFIX: &str = "#";
pub const FUNC_PREFIX: &str = "export.";
pub const MODULE_COMMENT_PREFIX: &str = "#!";

/// Holds a stdlib function name and comments for docs purposes.
pub struct Function {
    name: String,
    comments: Vec<String>,
}

/// Holds a stdlib module that contains a list of functions and comments.
pub struct Module {
    name: String,
    section: String,
    functions: Vec<Function>,
    comments: Vec<String>,
}

impl Module {
    pub fn new(ns: String) -> Self {
        let parts: Vec<&str> = ns.split("::").collect();
        let module_name = parts[parts.len() - 1];
        let section_name = parts[parts.len() - 2];
        Module {
            name: String::from(module_name),
            section: String::from(section_name),
            functions: Vec::new(),
            comments: Vec::new(),
        }
    }

    pub fn markdown_file_name(&self) -> String {
        format!("{}_{}.md", self.name, self.section)
    }
}

/// Holds the stdlib modules and corresponding functions.
pub struct Stdlib {
    modules: BTreeMap<String, Module>
}

impl Stdlib {
    pub fn new() -> Self {
        Stdlib {
            modules: BTreeMap::new()
        }
    }
}


#[derive(PartialEq)]
enum AsmSourceState {
    Empty,
    Comment,
    Func,
    ModuleComment,
}

// Writes Miden standard library modules documentation markdown files based on the available modules and comments.
pub fn build_stdlib_docs(module_map: &ModuleMap, doc_functions_path: &str) {
    let mut stdlib = Stdlib::new();
    // Parse source strings to populate modules struct
    for (ns, source) in module_map {
        parse_module(ns.clone(), source.clone(), &mut stdlib);
    }

    // Render the modules into markdown
    write_markdown(&stdlib, doc_functions_path);
}

// Generates the markdown files in the functions directory and update the index
// files for mdbook to include in the doc generation.
fn write_markdown(stdlib: &Stdlib, doc_functions_path: &str) {
    // Remove functions folder to re-generate
    fs::remove_dir_all(doc_functions_path).unwrap();
    fs::create_dir(doc_functions_path).unwrap();

    // Write per module markdown file
    for (ns, module) in &stdlib.modules {
        let file_name = module.markdown_file_name();
        let file_path = Path::new(doc_functions_path).join(file_name);
        println!("{}", file_path.as_os_str().to_str().unwrap());
        let mut f = fs::OpenOptions::new()
            .write(true)
            .append(true) // This is needed to append to file
            .create(true)
            .open(file_path)
            .expect("unable to open stdlib markdown file");

        // Render modules into markdown
        f.write_all(module.comments.join("\n").as_bytes())
            .expect("unable to write module comments");
        let header = format!(
            "\n## {}\n| Instruction | Description | \n| ----------- | ------------- |\n",
            ns
        );
        f.write_all(header.as_bytes())
            .expect("unable to write header to writer");
        for func in module.functions.iter() {
            let func_output = format!(
                "| {} | {} |\n",
                func.name,
                func.comments.join("<br />").replace('|', "\\|")
            );
            f.write_all(func_output.as_bytes())
                .expect("unable to write func to writer");
        }
    }

    let index_file_path = Path::new(doc_functions_path).join("main.md");
    let mut index_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(index_file_path)
        .expect("unable to open stdlib markdown index file");


    // Write module index markdown.
    // index markdown is included in the stdlib main page for accessing each module.
    for (ns, module) in &stdlib.modules {
        index_file
            .write_all(
                format!("- [{}](./{})\n", ns, module.markdown_file_name()).as_bytes(),
            )
            .expect("unable to write file name to index");
    }
}

// Parses the namespace and source file into stdlib Module.
fn parse_module(ns: String, source: String, stdlib: &mut Stdlib) {
    let current_state = AsmSourceState::Empty;
    let mut comments = Vec::<String>::new();

    let module = stdlib.modules
        .entry(ns.clone())
        .or_insert_with(|| Module::new(ns.clone()));

    for line in source.lines() {
        let new_state = parse_new_state(line);
        if new_state != current_state {
            match new_state {
                AsmSourceState::Func => {
                    let func_name = remove_prefix(FUNC_PREFIX, line);
                    module.functions.push(Function {
                        name: func_name,
                        comments: comments.clone(),
                    });
                    comments.clear();
                }
                AsmSourceState::Comment => comments.push(remove_prefix(COMMENT_PREFIX, line)),
                AsmSourceState::Empty => comments.clear(),
                AsmSourceState::ModuleComment => module
                    .comments
                    .push(remove_prefix(MODULE_COMMENT_PREFIX, line)),
            }
        } else {
            match new_state {
                AsmSourceState::Comment => comments.push(remove_prefix(COMMENT_PREFIX, line)),
                AsmSourceState::Empty => comments.clear(),
                _ => (),
            }
        }
    }
}

// HELPER FUNCTIONS
// ================================================================================================

fn parse_new_state(line: &str) -> AsmSourceState {
    if line.starts_with(MODULE_COMMENT_PREFIX) {
        AsmSourceState::ModuleComment
    } else if line.starts_with(COMMENT_PREFIX) {
        AsmSourceState::Comment
    } else if line.starts_with(FUNC_PREFIX) {
        AsmSourceState::Func
    } else {
        AsmSourceState::Empty
    }
}

fn remove_prefix(prefix: &str, line: &str) -> String {
    String::from(line.strip_prefix(prefix).unwrap())
}
