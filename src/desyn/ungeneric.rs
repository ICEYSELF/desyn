use std::fmt::Formatter;
use std::fmt::Result;

use syn::{GenericArgument, TypeParamBound};

use crate::desyn::DeSyntax;

impl DeSyntax for GenericArgument {
    fn de_syntax(&self, output: &mut Formatter<'_>) -> Result {
        match self {
            GenericArgument::Lifetime(lt) => write!(output, "Lifetime('{})", lt.ident),
            GenericArgument::Type(ty) => {
                write!(output, "Type(")?;
                ty.de_syntax(output)?;
                write!(output, ")")
            },
            GenericArgument::Const(_e) => write!(output, "Const(?)"), // TODO expression
            GenericArgument::Binding(binding) => {
                write!(output, "Binding({} = ", binding.ident)?;
                binding.ty.de_syntax(output)?;
                write!(output, ")")
            },
            GenericArgument::Constraint(c) => {
                write!(output, "Constraint({}, [", c.ident)?;
                for bound in c.bounds.iter() {
                    bound.de_syntax(output)?;
                    write!(output, ",")?;
                }
                write!(output, "])")
            }
        }
    }
}

impl DeSyntax for TypeParamBound {
    fn de_syntax(&self, output: &mut Formatter<'_>) -> Result {
        match self {
            TypeParamBound::Trait(_t) => {
                write!(output, "Trait(?)")
            },
            TypeParamBound::Lifetime(lt) => write!(output, "Lifetime('{})", lt.ident)
        }
    }
}
