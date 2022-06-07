use {crate::*, ::std::process::Command};

pub fn compile_and_run(path: PathBuf, body: String, args: &[OsString]) -> Result<()> {
    let mtime = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64();

    let hash = hashing::git_blob_sha1_hex(body.as_bytes());
    let hash8 = &hash[..8];

    let identifier = path.as_path().file_stem().unwrap().to_string_lossy().to_snake_case();
    let kebab = identifier.to_kebab_case();

    let mut manifest = format!(
        r#"
            [package]
            autobins = false
            edition = "2021"
            name = "{kebab}-{hash8}"
            publish = false
            version = "0.0.0"

            [[bin]]
            name = {identifier:?}
            path = "src/main.rs"

            [dependencies]
        "#
    );

    let file = ::syn::parse_file(&body)?;
    let mut crate_doc = String::new();

    for attr in file.attrs.iter() {
        if attr.path.is_ident("docs") {
            if let Ok(syn::Meta::NameValue(meta)) = attr.parse_meta() {
                if let syn::Lit::Str(lit) = meta.lit {
                    crate_doc.push_str(&lit.value());
                    crate_doc.push('\n');
                }
            }
        }
    }

    let root_crates = {
        impl<'ast> syn::visit::Visit<'ast> for Visitor {
            fn visit_path(&mut self, path: &'ast syn::Path) {
                if path.leading_colon.is_some() {
                    let root_crate = path.segments.first().unwrap().ident.to_string();
                    self.root_crates.insert(root_crate.clone());
                }
                syn::visit::visit_path(self, path);
            }

            fn visit_item_use(&mut self, item_use: &'ast syn::ItemUse) {
                if item_use.leading_colon.is_some() {
                    match &item_use.tree {
                        syn::UseTree::Path(syn::UsePath { ident, .. })
                        | syn::UseTree::Name(syn::UseName { ident, .. })
                        | syn::UseTree::Rename(syn::UseRename { ident, .. }) => {
                            self.root_crates.insert(ident.to_string());
                        }
                        syn::UseTree::Group(group) =>
                            for tree in group.items.iter() {
                                match tree {
                                    syn::UseTree::Path(syn::UsePath { ident, .. })
                                    | syn::UseTree::Name(syn::UseName { ident, .. })
                                    | syn::UseTree::Rename(syn::UseRename { ident, .. }) => {
                                        self.root_crates.insert(ident.to_string());
                                    }
                                    syn::UseTree::Glob(_) => todo!(),
                                    syn::UseTree::Group(_) => todo!(),
                                }
                            },
                        syn::UseTree::Glob(_) => {
                            eprintln!("This is weird and unexpected: {item_use:?}.");
                        }
                    }
                } else {
                    match &item_use.tree {
                        syn::UseTree::Group(group) => {
                            for tree in group.items.iter() {
                                // we need to support cases like
                                // use {{{::{{{{crossterm::style::
                                // {{{{Stylize}}}}}}}}}}};
                            }
                        }
                        _ => {}
                    }
                }
                syn::visit::visit_item_use(self, item_use);
            }
        }
        #[derive(Default)]
        struct Visitor {
            root_crates: std::collections::BTreeSet<String>,
        }
        let mut visitor = Visitor::default();
        syn::visit::visit_file(&mut visitor, &file);

        visitor.root_crates
    };

    let builtin_crates: std::collections::BTreeSet<String> =
        ["core", "alloc", "std", "proc_macro", "test"].iter().map(|s| s.to_string()).collect();

    for root_crate in root_crates {
        if builtin_crates.contains(&root_crate) {
            continue;
        }

        manifest.push_str(&format!("{root_crate} = \"*\"\n"));
    }

    let dir = tempfile::TempDir::new()?;
    let manifest_path = dir.path().join("Cargo.toml");
    std::fs::write(&manifest_path, manifest)?;
    let main_path = dir.path().join("src/main.rs");
    std::fs::create_dir_all(main_path.parent().unwrap())?;
    std::fs::write(main_path, body)?;

    Command::new("cargo").args(&["build", "--quiet"]).current_dir(dir.path()).status()?;

    std::process::exit(
        Command::new(format!("target/debug/{identifier}"))
            .args(args)
            .current_dir(dir.path())
            .status()?
            .code()
            .unwrap_or(0xFF),
    )
}
