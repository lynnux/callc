#![feature(plugin_registrar, box_syntax, rustc_private)]

extern crate rustc;
extern crate syntax;

use syntax::ext::base::{ExtCtxt, ItemModifier, Modifier};
use syntax::ast::{Item, MetaItem};
use syntax::ast;
use syntax::parse::token;
use rustc::plugin::Registry;
use syntax::ptr::P as AstPtr;
use syntax::codemap::Span;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(token::intern("stdcall_win"), Modifier(box CallCModifier));
}

struct CallCModifier;

#[cfg(target_os="windows")]
fn is_windows() -> bool { true }

#[cfg(not(target_os="windows"))]
fn is_windows() -> bool { false }

impl ItemModifier for CallCModifier {
    fn expand(&self, ecx: &mut ExtCtxt, span: Span, meta_item: &MetaItem, item: AstPtr<Item>) -> AstPtr<Item> {
        if is_windows() {
            item.map(move |from| if let ast::ItemFn(decl, unsafety, _, generics, block) = from.node {
                Item {
                    ident: from.ident,
                    attrs: from.attrs,
                    id: from.id,
                    node: ast::ItemFn(decl, unsafety, syntax::abi::Stdcall, generics, block),
                    vis: from.vis,
                    span: from.span
                }
            } else {
                from
            })
        } else { item }
    }
}
