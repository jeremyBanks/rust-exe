#![allow(dead_code)]

use crate::*;

pub fn help() -> Result<()> {
    println!("#!/usr/bin/env rust");

    std::process::exit(0)
}

pub fn run(path: PathBuf, args: &[OsString]) -> Result<()> {
    let body = std::fs::read_to_string(&path).unwrap();

    compile_and_run(path, body, args)
}

pub fn eval(body: String, args: &[OsString]) -> Result<()> {
    let body = format!("fn main() {{ println!(\"{{:#?}}\", {{{body}}}); }}");
    let hash = git_blob_sha1_hex(body.as_bytes());
    let path = current_dir()
        .unwrap()
        .join(format!("eval_{}.rs", &hash[..8]));

    compile_and_run(path, body, args)
}

#[derive(Debug, Clone)]
pub struct Input {
    working_dir: PathBuf,
    entry_point: Module,
}

impl Input {
    pub fn new(working_dir: PathBuf, entry_point: PathBuf) -> Input {
        Input {
            working_dir,
            entry_point: Module::from_path(entry_point),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    path: Option<PathBuf>,
    name: Option<String>,
    content: Option<ModuleData>,
}

impl Module {
    pub fn from_path(path: PathBuf) -> Module {
        Module {
            path: Some(path),
            name: None,
            content: None,
        }
    }

    pub fn from_name(name: String) -> Module {
        Module {
            path: None,
            name: Some(name),
            content: None,
        }
    }

    pub fn try_from_source(source: &str) -> Result<Module> {
        Ok(Module {
            path: None,
            name: None,
            content: Some(source.parse()?),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ModuleData {
    source: String,
    ast: syn::File,
    children: Vec<Module>,
    dependencies: Vec<Dependency>,
}

impl FromStr for ModuleData {
    type Err = eyre::Report;

    fn from_str(source: &str) -> Result<ModuleData> {
        let ast = syn::parse_file(source)?;

        let children = Vec::new();
        let dependencies = Vec::new();

        let source = source.to_string();

        Ok(ModuleData {
            ast,
            source,
            children,
            dependencies,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Dependency {
    ident: syn::Ident,
    unambiguous: bool,
    spec: Toml,
}
