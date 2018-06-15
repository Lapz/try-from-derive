#![crate_type = "proc-macro"]
#![recursion_limit = "192"]
// pub trait TryFrom<T>: Sized {
//     /// The type returned in the event of a conversion error.
//     type Error:Default;
//     /// Performs the conversion.
//     fn try_from(value:T) -> Result<Self, Self::Error>;
// }

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use syn::{Attribute, Data, DeriveInput, Expr, Ident, Lit, Meta, NestedMeta};

#[proc_macro_derive(TryFrom, attributes(TryFrom))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).expect("Couldn't parse item");
    let name = input.ident;

    let repr = find_repr(&input.attrs).expect("Expected repr attribute");
    // let err_ty = 

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let match_arms = gen_match_arm(&name,&repr, &input.data); 
    let expanded = quote! {
        impl #impl_generics ::TryFrom<#repr> for  #name #ty_generics #where_clause {
            fn try_from(n:#repr) -> Result<(#name),()> {
                match n {
                    #match_arms,
                    _ => Err(()),
                }
            }
        }
    };

       expanded.into()

    // unimplemented!()
}

fn find_repr(attributes: &[Attribute]) -> Option<Ident> {
    for attr in attributes {
        if let Some(ref meta) = attr.interpret_meta() {
            match *meta {
                Meta::Word(_) | Meta::NameValue(_) => (),
                Meta::List(ref list) => {
                    if list.ident.to_string() == "repr" {
                        let nested_meta = list.nested.first().unwrap();
                        let nested_meta = nested_meta.value();

                        match nested_meta {
                            NestedMeta::Meta(ref meta) => match *meta {
                                Meta::Word(ref ident) => return Some(ident.clone()),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }
    }

    None
}

fn gen_match_arm(ident:&Ident,repr: &Ident, data: &Data) -> proc_macro2::TokenStream {
    match *data {
        Data::Enum(ref e_num) => {
            let mut discr = None;

            let match_arms = e_num.variants.iter().enumerate().map(|(i,v)| {
                let v_name = &v.ident;

                if let Some((_, Expr::Lit(ref lit))) = v.discriminant {
                    match lit.lit {
                        Lit::Int(ref lit) => {
                            discr = Some(lit.value());
                            


                        }
                        _ => unreachable!(),
                    }
                
                }else {
                    discr = Some(i as u64);
                }
                

                match repr.to_string().as_ref() {
                    "u8" => {
                        let discr = discr.unwrap() as u8;
                        quote!(#discr => Ok(#ident::#v_name))
                    }
                    "u16" => {
                        let discr = discr.unwrap() as u16;
                        quote!(#discr => Ok(#ident::#v_name))
                    }
                    "u32" => {
                        let discr = discr.unwrap() as u32;
                        quote!(#discr => Ok(#ident::#v_name))
                    }
                    "u64" => {
                        let discr = discr.unwrap() as u64;
                        quote!(#discr => Ok(#ident::#v_name))
                    }
                    "usize" => {
                        let discr = discr.unwrap() as usize;
                        quote!(#discr=> Ok(#ident::#v_name))
                    }
                    "i8" => {
                        let discr = discr.unwrap() as i8;
                        quote!(#discr => Ok(#ident::#v_name))
                    }
                    "i16" => {
                        let discr = discr.unwrap() as i16;
                        quote!(#discr=> Ok(#ident::#v_name))
                    }
                    "i32" => {
                        let discr = discr.unwrap() as i32;
                        quote!(#discr => Ok(#ident::#v_name))
                    }
                    "i64" => {
                        let discr = discr.unwrap() as i64;
                        quote!(#discr => Ok(#ident::#v_name))
                    }
                    "isize" => {
                        let discr = discr.unwrap() as isize;
                        quote!(#discr => Ok(#ident::#v_name))
                    }
                    ty => {
                        panic!(
                            "#[derive(TryFrom)] does not support enum repr type {:?}",
                            ty
                        );
                    }
                }
            });

            return quote![#(#match_arms),*]
        }

        _ => unimplemented!(),
    }
    
}
