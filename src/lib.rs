#![feature(plugin_registrar, box_syntax, rustc_private)]

extern crate rustc;
extern crate syntax;

use syntax::ext::base::{ExtCtxt, MultiItemModifier, MultiModifier, Annotatable};
use syntax::ast::{Item, MetaItem};
use syntax::ast;
use syntax::parse::token;
use rustc::plugin::Registry;
use syntax::codemap::Span;

#[derive(PartialEq, Eq)]
enum Platform {
    Windows,
    Mac,
    Linux
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    let platform = match &*reg.sess.target.target.target_os {
        "windows" => Platform::Windows,
        "macos" => Platform::Mac,
        "linux" => Platform::Linux,
        other => panic!("Sorry, platform \"{}\" is not supported by cef-sys.", other)
    };
    reg.register_syntax_extension(token::intern("stdcall_win"), MultiModifier(box CallCModifier{ platform: platform }));
}

struct CallCModifier {
    platform: Platform
}

impl MultiItemModifier for CallCModifier {
    fn expand(&self, _: &mut ExtCtxt, _: Span, _: &MetaItem, item: Annotatable) -> Annotatable {
        match item {
            Annotatable::Item(item) =>
                Annotatable::Item(if self.platform == Platform::Windows {
                    item.map(
                        move |from|
                        if let ast::ItemFn(decl, unsafety, constness, _, generics, block) = from.node {
                            Item {
                                ident: from.ident,
                                attrs: from.attrs,
                                id: from.id,
                                node: ast::ItemFn(
                                    decl,
                                    unsafety,
                                    constness,
                                    syntax::abi::Stdcall,
                                    generics,
                                    block),
                                vis: from.vis,
                                span: from.span
                            }
                        } else {
                            from
                        })
                } else { item }),
            other => other
        }
    }
}
