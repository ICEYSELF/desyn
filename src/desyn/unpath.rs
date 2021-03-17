use std::fmt::Formatter;
use std::fmt::Result;

use syn::{Path, PathArguments, ReturnType, TypePath};

use crate::desyn::DeSyntax;

impl DeSyntax for Path {
    fn de_syntax(&self, output: &mut Formatter<'_>) -> Result {
        write!(output, "Path(")?;
        if self.leading_colon.is_some() {
            write!(output, "::")?;
        }
        for segment in self.segments.iter() {
            write!(output, "Seg({}),", segment.ident.to_string())?;
            if !segment.arguments.is_empty() {
                segment.arguments.de_syntax(output)?;
                write!(output, ",")?;
            }
        }
        write!(output, ")")
    }
}

impl DeSyntax for TypePath {
    fn de_syntax(&self, output: &mut Formatter<'_>) -> Result {
        write!(output, "TypePath(")?;
        if let Some(qself) = self.qself.as_ref() {
            write!(output, "QSelf(")?;
            qself.ty.de_syntax(output)?;
            write!(output, " as ")?;
            self.path.de_syntax(output)?;
            write!(output, "))")
        } else {
            self.path.de_syntax(output)?;
            write!(output,")")
        }
    }
}

impl DeSyntax for PathArguments {
    fn de_syntax(&self, output: &mut Formatter<'_>) -> Result {
        write!(output, "PathArgs(")?;
        match self {
            PathArguments::None => {},
            PathArguments::Parenthesized(args) => {
                write!(output, "Parenthesized((")?;
                for arg in args.inputs.iter() {
                    arg.de_syntax(output)?;
                    write!(output, ",")?;
                }
                if let ReturnType::Type(_, ty) = &args.output {
                    write!(output, ") -> ")?;
                    ty.de_syntax(output)?;
                } else {
                    write!(output, ")")?;
                }
            },
            PathArguments::AngleBracketed(args) => {
                write!(output, "AngleBracketed(")?;
                for arg in args.args.iter() {
                    arg.de_syntax(output)?;
                    write!(output, ",")?;
                }
                write!(output, ")")?;
            }
        }
        write!(output, ")")
    }
}
