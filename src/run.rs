#!/usr/bin/env rust
//![#!args]: --verbose
//![#!date]: ~2022-12-09-16:09:53
//![#!date]: ~2020
//![#!rust]: >=1.59.0
//!
///[#!syn]: =0.1.0 (-default +std)
pub mod foo {}

///[#!std]: 0.1.0 (-default +std)
///[#!std]: toml ({ version = "1.2.4", path = "foo/bar" })
///[#!syn]: =0.1.0 (-default +std)
use ::{std, syn};
use {
    crate::*,
    ::std::{fs, process::Command},
};

pub fn compile_and_run(path: PathBuf, body: String, args: &[OsString]) -> Result<()> {
    let data_dir = ::home::home_dir().unwrap_or_default().join(".rust-exe");
    let src_dir = data_dir.join("src");
    let tmp_dir = data_dir.join("tmp");
    let bin_dir = data_dir.join("bin");

    fs::create_dir_all(&src_dir).unwrap();
    fs::create_dir_all(&tmp_dir).unwrap();
    fs::create_dir_all(&bin_dir).unwrap();

    let _mtime = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64();

    let hash = hashing::git_blob_sha1_hex(body.as_bytes());
    let hash8 = &hash[..8];

    let path_hash = hashing::git_blob_sha1_hex(path.as_os_str().as_bytes());
    let path8 = &path_hash[..8];

    let name = path.as_path().file_stem().unwrap().to_string_lossy();
    let snake = name.to_snake_case();
    let kebab = name.to_kebab_case();
    let filename = format!("{snake}.rs");

    let version = format!("0.0.0-{hash8}");

    let crate_name = format!("{kebab}-{path8}");
    let crate_path = src_dir.join(&crate_name);

    fs::remove_dir_all(&crate_path).ok();
    fs::create_dir_all(&crate_path).unwrap();

    let mut manifest = toml! {
        [package]
        autobins = false
        edition = "2021"
        name = (crate_name.clone())
        version = version

        [[bin]]
        name = (crate_name.clone())
        path = (filename.clone())

        [dependencies]
    };

    let file = syn::parse_file(&body)?;
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
                    self.root_crates.insert(root_crate);
                }
                syn::visit::visit_path(self, path);
            }

            fn visit_item_extern_crate(&mut self, item: &'ast syn::ItemExternCrate) {
                if item.rename.is_some() {
                    todo!("extern crate with rename not supported");
                }
                self.root_crates.insert(item.ident.to_string());
            }

            fn visit_item_use(&mut self, item_use: &'ast syn::ItemUse) {
                if item_use.leading_colon.is_some() {
                    match &item_use.tree {
                        syn::UseTree::Path(syn::UsePath { ident, .. })
                        | syn::UseTree::Name(syn::UseName { ident, .. })
                        | syn::UseTree::Rename(syn::UseRename { ident, .. }) => {
                            self.root_crates.insert(ident.to_string());
                        }
                        syn::UseTree::Group(group) => {
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
                            }
                        }
                        syn::UseTree::Glob(_) => {
                            eprintln!("This is weird and unexpected: {item_use:?}.");
                        }
                    }
                } else {
                    match &item_use.tree {
                        syn::UseTree::Group(group) => {
                            for _tree in group.items.iter() {
                                // we need to support cases like
                                // use {{{::{{{{crossterm::style::
                                // {{{{Stylize}}}}}}}}}}};
                            }
                        }
                        syn::UseTree::Name(_) => {}
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
        ["core", "alloc", "std", "proc_macro", "test"]
            .iter()
            .map(|s| s.to_string())
            .collect();

    for root_crate in root_crates {
        if builtin_crates.contains(&root_crate) {
            continue;
        }

        manifest["dependencies"].as_table_mut().unwrap().insert(
            root_crate.clone(),
            toml! {
                version = "*"
            },
        );
    }

    let manifest_path = crate_path.join("Cargo.toml");
    std::fs::write(&manifest_path, manifest.to_string())?;
    let main_path = crate_path.join(filename);
    std::fs::write(main_path, body)?;

    Command::new("cargo")
        .args(&["build", "--quiet", "--target-dir"])
        .arg(&tmp_dir)
        .current_dir(&crate_path)
        .status()?;

    let lockfile = Lockfile::load(crate_path.join("Cargo.lock")).unwrap();
    let package_lock = &lockfile
        .packages
        .iter()
        .find(|p| p.name.as_str() == crate_name)
        .unwrap();
    let _dependencies = &package_lock
        .dependencies
        .iter()
        .map(|d| format!("{} {}", d.name, d.version))
        .collect::<Vec<_>>();

    std::fs::copy(
        tmp_dir.join("debug").join(&crate_name),
        bin_dir.join(&crate_name),
    )?;

    let status = Command::new(bin_dir.join(&crate_name))
        .args(args)
        .status()?
        .code()
        .unwrap_or(0xFF);

    Command::new("find")
        .arg(src_dir)
        .args(["-mmin", "32", "-delete"])
        .status()?;
    Command::new("find")
        .arg(tmp_dir)
        .args(["-atime", "2", "-delete"])
        .status()?;
    Command::new("find")
        .arg(bin_dir)
        .args(["-atime", "8", "-delete"])
        .status()?;

    std::process::exit(status);
}
