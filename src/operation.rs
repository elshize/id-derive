use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{Data, DeriveInput, Fields, FieldsUnnamed, Ident, Type};

fn implement_operation<F>(name: &Ident, fields: &FieldsUnnamed, operation: F) -> TokenStream
where
    F: Fn(&Ident, &Type) -> TokenStream,
{
    if fields.unnamed.len() > 1 {
        quote_spanned! {name.span()=>
            compile_error!("Only single-field structs supported at the moment");
        }
    } else {
        let field = &fields.unnamed.first().unwrap();
        let field_ty = &field.ty;
        operation(name, field_ty)
    }
}

fn single_derive<F>(
    derive_name: &'static str,
    input: &DeriveInput,
    operation: F,
) -> syn::Result<TokenStream>
where
    F: Fn(&Ident, &Type) -> TokenStream,
{
    let name = &input.ident;
    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Unnamed(fields) => Ok(implement_operation(name, fields, operation)),
            Fields::Unit => Err(syn::Error::new(
                name.span(),
                format!("Unit struct cannot derive {}", derive_name),
            )),
            _ => Err(syn::Error::new(
                name.span(),
                format!(
                    "Deriving from {} by a struct with named fields is not yet implemented",
                    derive_name
                ),
            )),
        },
        Data::Enum(data) => Err(syn::Error::new(
            data.enum_token.span,
            format!("Cannot derive {} for enum, expected struct.", derive_name),
        )),
        Data::Union(data) => Err(syn::Error::new(
            data.union_token.span,
            format!("Cannot derive {} for union, expected struct.", derive_name),
        )),
    }
}

pub fn into_inner(derive_name: &'static str, input: &DeriveInput) -> syn::Result<TokenStream> {
    single_derive(derive_name, input, |name, ty| {
        quote! {
            impl ::std::convert::From<#name> for #ty {
                fn from(inner: #name) -> Self {
                    inner.0
                }
            }
        }
    })
}

pub fn from_inner(derive_name: &'static str, input: &DeriveInput) -> syn::Result<TokenStream> {
    single_derive(derive_name, input, |name, ty| {
        quote! {
            impl ::std::convert::From<#ty> for #name {
                fn from(inner: #ty) -> Self {
                    Self(inner)
                }
            }
        }
    })
}

pub fn mul_self(derive_name: &'static str, input: &DeriveInput) -> syn::Result<TokenStream> {
    single_derive(derive_name, input, |name, _| {
        quote! {
            impl ::std::ops::Mul for #name {
                type Output = Self;
                fn mul(self, rhs: Self) -> Self::Output {
                    Self(self.0 * rhs.0)
                }
            }
        }
    })
}

pub fn mul_inner(derive_name: &'static str, input: &DeriveInput) -> syn::Result<TokenStream> {
    single_derive(derive_name, input, |name, ty| {
        quote! {
            impl ::std::ops::Mul<#ty> for #name {
                type Output = Self;
                fn mul(self, rhs: #ty) -> Self::Output {
                    Self(self.0 * rhs)
                }
            }
        }
    })
}

pub fn mul_assign_self(derive_name: &'static str, input: &DeriveInput) -> syn::Result<TokenStream> {
    single_derive(derive_name, input, |name, _| {
        quote! {
            impl ::std::ops::MulAssign for #name {
                fn mul_assign(&mut self, rhs: Self) {
                    self.0 *= rhs.0;
                }
            }
        }
    })
}

pub fn mul_assign_inner(
    derive_name: &'static str,
    input: &DeriveInput,
) -> syn::Result<TokenStream> {
    single_derive(derive_name, input, |name, ty| {
        quote! {
            impl ::std::ops::MulAssign<#ty> for #name {
                fn mul_assign(&mut self, rhs: #ty) {
                    self.0 *= rhs;
                }
            }
        }
    })
}

pub fn div_self(derive_name: &'static str, input: &DeriveInput) -> syn::Result<TokenStream> {
    single_derive(derive_name, input, |name, _| {
        quote! {
            impl ::std::ops::Div for #name {
                type Output = Self;
                fn div(self, rhs: Self) -> Self::Output {
                    Self(self.0 / rhs.0)
                }
            }
        }
    })
}

pub fn div_inner(derive_name: &'static str, input: &DeriveInput) -> syn::Result<TokenStream> {
    single_derive(derive_name, input, |name, ty| {
        quote! {
            impl ::std::ops::Div<#ty> for #name {
                type Output = Self;
                fn div(self, rhs: #ty) -> Self::Output {
                    Self(self.0 / rhs)
                }
            }
        }
    })
}

pub fn div_assign_self(derive_name: &'static str, input: &DeriveInput) -> syn::Result<TokenStream> {
    single_derive(derive_name, input, |name, _| {
        quote! {
            impl ::std::ops::DivAssign for #name {
                fn div_assign(&mut self, rhs: Self) {
                    self.0 /= rhs.0;
                }
            }
        }
    })
}

pub fn div_assign_inner(
    derive_name: &'static str,
    input: &DeriveInput,
) -> syn::Result<TokenStream> {
    single_derive(derive_name, input, |name, ty| {
        quote! {
            impl ::std::ops::DivAssign<#ty> for #name {
                fn div_assign(&mut self, rhs: #ty) {
                    self.0 /= rhs;
                }
            }
        }
    })
}

pub fn add_self(derive_name: &'static str, input: &DeriveInput) -> syn::Result<TokenStream> {
    single_derive(derive_name, input, |name, _| {
        quote! {
            impl ::std::ops::Add for #name {
                type Output = Self;
                fn add(self, rhs: Self) -> Self::Output {
                    Self(self.0 + rhs.0)
                }
            }
        }
    })
}

pub fn add_inner(derive_name: &'static str, input: &DeriveInput) -> syn::Result<TokenStream> {
    single_derive(derive_name, input, |name, ty| {
        quote! {
            impl ::std::ops::Add<#ty> for #name {
                type Output = Self;
                fn add(self, rhs: #ty) -> Self::Output {
                    Self(self.0 + rhs)
                }
            }
        }
    })
}

pub fn add_assign_self(derive_name: &'static str, input: &DeriveInput) -> syn::Result<TokenStream> {
    single_derive(derive_name, input, |name, _| {
        quote! {
            impl ::std::ops::AddAssign for #name {
                fn add_assign(&mut self, rhs: Self) {
                    self.0 += rhs.0
                }
            }
        }
    })
}

pub fn add_assign_inner(
    derive_name: &'static str,
    input: &DeriveInput,
) -> syn::Result<TokenStream> {
    single_derive(derive_name, input, |name, ty| {
        quote! {
            impl ::std::ops::AddAssign<#ty> for #name {
                fn add_assign(&mut self, rhs: #ty) {
                    self.0 += rhs
                }
            }
        }
    })
}

pub fn sub_self(derive_name: &'static str, input: &DeriveInput) -> syn::Result<TokenStream> {
    single_derive(derive_name, input, |name, _| {
        quote! {
            impl ::std::ops::Sub for #name {
                type Output = Self;
                fn sub(self, rhs: Self) -> Self::Output {
                    Self(self.0 - rhs.0)
                }
            }
        }
    })
}

pub fn sub_inner(derive_name: &'static str, input: &DeriveInput) -> syn::Result<TokenStream> {
    single_derive(derive_name, input, |name, ty| {
        quote! {
            impl ::std::ops::Sub<#ty> for #name {
                type Output = Self;
                fn sub(self, rhs: #ty) -> Self::Output {
                    Self(self.0 - rhs)
                }
            }
        }
    })
}

pub fn sub_assign_self(derive_name: &'static str, input: &DeriveInput) -> syn::Result<TokenStream> {
    single_derive(derive_name, input, |name, _| {
        quote! {
            impl ::std::ops::SubAssign for #name {
                fn sub_assign(&mut self, rhs: Self) {
                    self.0 -= rhs.0
                }
            }
        }
    })
}

pub fn sub_assign_inner(
    derive_name: &'static str,
    input: &DeriveInput,
) -> syn::Result<TokenStream> {
    single_derive(derive_name, input, |name, ty| {
        quote! {
            impl ::std::ops::SubAssign<#ty> for #name {
                fn sub_assign(&mut self, rhs: #ty) {
                    self.0 -= rhs
                }
            }
        }
    })
}

pub fn display(derive_name: &'static str, input: &DeriveInput) -> syn::Result<TokenStream> {
    single_derive(derive_name, input, |name, _| {
        quote! {
            impl ::std::fmt::Display for #name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>)
                    -> ::std::result::Result<(), ::std::fmt::Error>
                {
                    write!(f, "{}", self.0)
                }
            }
            impl ::std::fmt::Binary for #name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>)
                    -> ::std::result::Result<(), ::std::fmt::Error>
                {
                    if f.alternate() {
                        write!(f, "{:#b}", self.0)
                    } else {
                        write!(f, "{:b}", self.0)
                    }
                }
            }
        }
    })
}
