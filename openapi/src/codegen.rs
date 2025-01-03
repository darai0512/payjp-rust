use std::fmt::Write as _;
use std::path::PathBuf;

use indoc::formatdoc;

use crate::components::{get_components, Components};
use crate::crate_table::write_crate_table;
use crate::crates::{get_crate_doc_comment, Crate, ALL_CRATES};
use crate::object_writing::{gen_obj, gen_requests};
use crate::rust_object::{ObjectKind, ObjectMetadata};
use crate::spec::Spec;
use crate::spec_inference::infer_doc_comment;
use crate::resource_object::PayjpObject;
use crate::templates::cargo_toml::gen_crate_toml;
use crate::templates::utils::write_top_level_doc_comment;
use crate::utils::{append_to_file, write_to_file};

pub struct CodeGen {
    pub components: Components,
    pub spec: Spec,
}

impl CodeGen {
    pub fn new(spec: Spec) -> anyhow::Result<Self> {
        let components = get_components(&spec)?;

        Ok(Self { components, spec })
    }

    fn write_components(&self) -> anyhow::Result<()> {
        for component in self.components.components.values() {
            self.write_component(component)?;
        }

        let crate_path = Crate::SHARED.get_path();
        let crate_mod_path = crate_path.join("mod.rs");
        for (ident, typ_info) in &self.components.extra_types {
            let mut out = String::new();
            let metadata = ObjectMetadata::new(ident.clone());
            self.components.write_object(&typ_info.obj, &metadata, typ_info.usage, &mut out);
            write_to_file(out, crate_path.join(format!("{}.rs", typ_info.mod_path)))?;
            append_to_file(
                format!("pub mod {0}; pub use {0}::{1};", typ_info.mod_path, ident),
                &crate_mod_path,
            )?;
        }
        Ok(())
    }

    pub fn write_files(&self) -> anyhow::Result<()> {
        self.write_crate_base()?;
        self.write_components()?;
        write_crate_table(&self.components)?;
        self.write_object_info_for_testing()
    }

    fn write_crate_base(&self) -> anyhow::Result<()> {
        let crate_graph = self.components.gen_crate_dep_graph();

        for krate in &*ALL_CRATES {
            let neighbors = crate_graph.neighbors(*krate);
            let base_path = PathBuf::from(krate.generated_out_path());
            let request_features = self
                .components
                .get_crate_members(*krate)
                .into_iter()
                .filter(|c| !self.components.get(c).requests.is_empty() && *krate != Crate::SHARED)
                .map(|c| self.components.get(c).mod_path())
                .collect();

            let toml = gen_crate_toml(*krate, neighbors.collect(), request_features);
            write_to_file(toml, base_path.join("Cargo.toml"))?;

            let lib_name = krate.lib_name();
            let doc_comment = write_top_level_doc_comment(get_crate_doc_comment(*krate));

            // We set up some things in the base `mod.rs` file:
            // 1. Without this recursion limit increase, `cargo doc` fails
            // 2. The `extern` allows generated code to use absolute paths starting with the crate name instead of `crate`
            // 3. Allow some warnings that are not currently fixed, but could be.
            let mod_rs = formatdoc! {
                r#"
            #![recursion_limit = "256"]
            #![allow(clippy::large_enum_variant)]
            #![allow(rustdoc::broken_intra_doc_links)]
            #![allow(rustdoc::invalid_html_tags)]
            
            {doc_comment}
            extern crate self as {lib_name};

            miniserde::make_place!(Place);
            "#
            };

            let mod_path = base_path.join("src/mod.rs");
            write_to_file(mod_rs, &mod_path)?;

            // NB: a hack to avoid the insanely long lines generated because of _very_ long
            // type names causing `rustfmt` errors (https://github.com/rust-lang/rustfmt/issues/5315)
        }
        Ok(())
    }

    fn write_component_requests(&self, comp: &PayjpObject) -> anyhow::Result<()> {
        let req_content = gen_requests(&comp.requests, &self.components);

        let req_file_content = formatdoc! {
            r#"
            use payjp_client_core::{{PayjpClient, BlockingClient, PayjpRequest, RequestBuilder, PayjpMethod}};
            
            {req_content}
            "#
        };
        write_to_file(req_file_content, comp.get_requests_content_path())?;
        let feature_gate = comp.mod_path();
        let new_mod_file_content = formatdoc! {
            r#"
            #[cfg(feature = "{feature_gate}")]
            mod requests;
            #[cfg(feature = "{feature_gate}")]
            pub use requests::*;
            "#
        };
        append_to_file(new_mod_file_content, comp.get_requests_module_path())
    }

    #[tracing::instrument(level = "debug", skip_all, fields(path = %comp.path()))]
    fn write_component(&self, comp: &PayjpObject) -> anyhow::Result<()> {
        let struct_defs = self.gen_struct_definitions_for_component(comp);
        if !comp.has_requests() || comp.types_split_from_requests() {
            write_to_file(struct_defs, comp.get_types_content_path())?;
            append_to_file(
                format!(
                    // NB: we add doc(hidden) and doc(inline) to hide the implementation details
                    // of these being public modules so that other generated crates can import them. It
                    // just keeps the already giant rustdoc page a bit cleaner
                    "#[doc(hidden)]\npub mod {0};#[doc(inline)]\npub use {0}::*;",
                    comp.mod_path()
                ),
                comp.types_crate().get_path_to_mod(),
            )?;
        } else {
            write_to_file(struct_defs, comp.get_types_content_path())?;
            append_to_file("pub(crate) mod types;", comp.get_requests_module_path())?;
            append_to_file(
                format!("pub use {0}::types::*;", comp.mod_path()),
                comp.req_crate().get_path_to_mod(),
            )?;
        }

        if !comp.requests.is_empty() {
            append_to_file(
                format!("pub mod {};", comp.mod_path()),
                comp.req_crate().get_path_to_mod(),
            )?;
            self.write_component_requests(comp)?;
        }

        if comp.types_split_from_requests() {
            append_to_file(
                format!("pub use {}::{}::*;", Crate::SHARED.lib_name(), comp.mod_path()),
                comp.req_crate().get_path_to_mod(),
            )?;
        }

        for (ident, obj) in &comp.deduplicated_objects {
            let mut out = String::new();
            let metadata = ObjectMetadata::new(ident.clone());
            self.components.write_object(&obj.object, &metadata, obj.info.usage, &mut out);
            let dst_file = match obj.info.usage.kind {
                ObjectKind::RequestParam | ObjectKind::RequestReturned => {
                    comp.get_requests_content_path()
                }
                ObjectKind::Type => comp.get_types_content_path(),
            };
            append_to_file(out, dst_file)?;
        }

        Ok(())
    }

    fn gen_struct_definitions_for_component(&self, comp: &PayjpObject) -> String {
        let base_obj = comp.rust_obj();
        let schema = self.spec.get_component_schema(comp.path());
        let doc_comment = infer_doc_comment(schema);
        let meta = ObjectMetadata::new(comp.ident().clone()).doc(doc_comment);

        gen_obj(base_obj, &meta, comp, &self.components)
    }

    fn write_object_info_for_testing(&self) -> anyhow::Result<()> {
        let mut checks = String::new();
        for (path, obj) in &self.components.components {
            if obj.object_name().is_none() {
                continue;
            }
            let krate = obj.krate().unwrap().base();
            let import_path = format!("{}::{}", krate.lib_name(), obj.ident());
            let _ = writeln!(checks, r#"check_object::<{import_path}>(resources, "{path}");"#);
        }
        let content = formatdoc! {
            r#"
            use crate::deserialization_fixture::check_object;

            pub fn check_fixtures(resources: &serde_json::Value) {{
                {checks}
            }}
            "#
        };

        write_to_file(content, "tests/mod.rs")
    }
}
