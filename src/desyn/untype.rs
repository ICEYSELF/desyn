use std::fmt::Formatter;
use std::fmt::Result;

use syn::{ReturnType, Type, TypeArray, TypeBareFn, TypeImplTrait};

use crate::desyn::DeSyntax;

impl DeSyntax for Type {
    fn de_syntax(&self, output: &mut Formatter<'_>) -> Result {
        match self {
            Type::Array(array) => {
                write!(output, "ArrayType(")?;
                array.de_syntax(output)?;
                write!(output, ")")
            },
            Type::BareFn(bare_fn) => {
                write!(output, "BareFn(")?;
                bare_fn.de_syntax(output)?;
                write!(output, ")")
            },
            Type::Group(group) => {
                write!(output, "Group(")?;
                group.elem.de_syntax(output)?;
                write!(output, ")")
            },
            Type::ImplTrait(impl_trait) => {
                write!(output, "ImplTrait(")?;
                impl_trait.de_syntax(output)?;
                write!(output, ")")
            },
            Type::Infer(_) => {
                write!(output, "Inferred")
            },
            Type::Macro(_) => write!(output, "Macro(?)"),
            Type::Never(_) => {
                write!(output, "Never")
            },
            Type::Paren(paren) => {
                paren.elem.de_syntax(output)
            },
            Type::Path(path) => path.de_syntax(output),
            Type::Ptr(ptr) => {
                write!(output, "Ptr(")?;
                if ptr.const_token.is_some() {
                    write!(output, "const,")?;
                }
                if ptr.mutability.is_some() {
                    write!(output, "mut,")?;
                }
                ptr.elem.de_syntax(output)?;
                write!(output, ")")
            },
            Type::Reference(r) => {
                write!(output, "Ref({},{},",
                       r.lifetime.as_ref().map_or("'_".to_string(), |lt| lt.ident.to_string()),
                       if r.mutability.is_some() { "mut" } else { "(_const)" })?;
                r.elem.de_syntax(output)?;
                write!(output, ")")
            },
            Type::Slice(slice) => {
                write!(output, "Slice(")?;
                slice.elem.de_syntax(output)?;
                write!(output, ")")
            },
            Type::TraitObject(to) => {
                write!(output, "TraitObject(dyn ")?;
                for bound in to.bounds.iter() {
                    bound.de_syntax(output)?;
                    write!(output, "+")?;
                }
                write!(output, ")")
            },
            Type::Tuple(t) => {
                write!(output, "Tuple(")?;
                for elem in t.elems.iter() {
                    elem.de_syntax(output)?;
                    write!(output, ",")?;
                }
                write!(output, ")")
            },
            Type::Verbatim(v) => {
                write!(output, "Verbatim({:?})", v.to_string())
            },
            Type::__TestExhaustive(_) => unreachable!()
        }
    }
}

impl DeSyntax for TypeArray {
    fn de_syntax(&self, output: &mut Formatter<'_>) -> Result {
        write!(output, "Array(")?;
        self.elem.de_syntax(output)?;
        write!(output, ")")
    }
}

impl DeSyntax for TypeBareFn {
    fn de_syntax(&self, output: &mut Formatter<'_>) -> Result {
        write!(output, "fn(")?;
        for arg in self.inputs.iter() {
            // TODO handle attributes
            arg.ty.de_syntax(output)?;
        }
        if let ReturnType::Type(_, ty) = &self.output {
            write!(output, ") -> ")?;
            ty.de_syntax(output)
        } else {
            write!(output, ")")
        }
    }
}

impl DeSyntax for TypeImplTrait {
    fn de_syntax(&self, output: &mut Formatter<'_>) -> Result {
        write!(output, "impl ")?;
        for _bound in self.bounds.iter() {
            todo!();
        }
        todo!()
    }
}
