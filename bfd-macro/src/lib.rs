use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Fields)]
pub fn fields_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let layout = generate_layout(input);
    layout.into()
}

fn generate_layout(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let struct_ident = ast.ident;
    if struct_ident.to_string().contains("Meta") == false {
        core::panic!("The struct derived Bdf should be named postfix Meta")
    }
    let fields_trait_name = format_ident!(
        "{}Fields",
        struct_ident.to_string().strip_suffix("Meta").unwrap()
    );
    let struct_plain_name =
        format_ident!("{}", struct_ident.to_string().strip_suffix("Meta").unwrap());
    let mut fields_id = Vec::<syn::Ident>::new();
    let mut fields_ty = Vec::<syn::Type>::new();
    if let syn::Data::Struct(data) = ast.data {
        data.fields.iter().for_each(|x| {
            fields_id.push(x.ident.to_owned().unwrap());
            fields_ty.push(x.ty.to_owned());
        });
    }
    let mut fields_range = Vec::<proc_macro2::TokenStream>::new();
    let first_ty = fields_ty[0].to_owned();
    fields_range.push(quote!(0..core::mem::size_of::<#first_ty>()));
    fields_id[0..]
        .iter()
        .zip(fields_ty[1..].iter())
        .for_each(|(id,ty)| {
            fields_range.push(quote!(<#id>::layout_range().end..<#id>::layout_range().end + core::mem::size_of::<#ty>()))
        });

    let struct_size = fields_ty.iter().fold(quote!(0), |mut acc, ty| {
        acc.extend(quote!(+ core::mem::size_of::<#ty>()));
        acc
    });
    quote! {
        mod bfd {
            use core::convert::TryInto;
            pub struct #struct_plain_name<'a, End: crate::Endianess> {
                raw: &'a [u8; #struct_size],
                phantom: core::marker::PhantomData<End>
            }
            impl<'a> #struct_plain_name<'a, crate::Le> {
                pub fn get<T: fields::#fields_trait_name + crate::ByteOrder<'a>>(&self)-> T {
                    T::from_le_bytes((&self.raw[T::layout_range()]).try_into().unwrap())
                }
                // pub fn to_meta(&self)-> super::#struct_ident {
                //     super::#struct_ident {
                //         #(
                //             #fields_id: self.get::<#fields_id>(),
                //         )*
                //     }
                // }
            }
            pub mod fields {
                use core::convert::TryInto;
                pub trait #fields_trait_name {
                    fn layout_range()->core::ops::Range<usize>;
                }
                #(
                    #[allow(non_camel_case_types)]
                    pub struct #fields_id {
                        value: #fields_ty
                    }
                    impl #fields_trait_name for #fields_id{
                        #[inline]
                        fn layout_range()->core::ops::Range<usize> {
                            #fields_range
                        }
                    }
                    impl #fields_id {
                        fn new(value: #fields_ty)-> #fields_id {
                            #fields_id {
                                value
                            }
                        }
                    }
                    impl<'a> crate::ByteOrder<'a> for #fields_id {
                        type Bytes = [u8; core::mem::size_of::<#fields_ty>()];
                        fn to_ne_bytes(&self) -> [u8; core::mem::size_of::<#fields_ty>()] {
                            #fields_ty::to_ne_bytes(self.value)
                        }
                        fn to_le_bytes(&self) -> [u8; core::mem::size_of::<#fields_ty>()] {
                            #fields_ty::to_le_bytes(self.value)
                        }
                        fn to_be_bytes(&self) -> [u8; core::mem::size_of::<#fields_ty>()] {
                            #fields_ty::to_be_bytes(self.value)
                        }
                        fn from_le_bytes(x: Self::Bytes) -> Self {
                            Self {value:#fields_ty::from_le_bytes(x)}
                        }
                        fn from_be_bytes(x: &[u8]) -> Self {
                            Self {value:#fields_ty::from_be_bytes(x.try_into().unwrap())}
                        }
                        fn from_ne_bytes(x: &[u8]) -> Self {
                            Self {value:#fields_ty::from_ne_bytes(x.try_into().unwrap())}
                        }
                        fn from_be(x: Self) -> Self {
                            Self {
                                value: #fields_ty::from_be(x.value)
                            }
                        }
                        fn from_le(x: Self) -> Self {
                            Self {
                                value: #fields_ty::from_le(x.value)
                            }
                        }
                        fn to_be(self) -> Self {
                            Self {
                                value: #fields_ty::to_be(self.value)
                            }
                        }
                        fn to_le(self) -> Self {
                            Self {
                                value: #fields_ty::to_le(self.value)
                            }
                        }
                    }
                )*
            }
        }
    }
}
