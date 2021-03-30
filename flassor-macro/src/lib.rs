use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Fields)]
pub fn fields_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let layout = gen_fields(input);
    layout.into()
}

#[proc_macro_attribute]
pub fn field_accessor(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    todo!()
}

fn gen_fields(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let struct_ident = ast.ident;
    let fields_trait_name = format_ident!(
        "{}Fields",
        struct_ident.to_string()
    );
    let struct_plain_name =
        format_ident!("{}Flat", struct_ident.to_string());
    let struct_plain_mut_name = format_ident!("{}Mut", struct_plain_name);

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
        use crate::flassor::Endianess;

        impl Into<[u8; #struct_ident::plain_size]> for #struct_ident {
            fn into(self)->[u8; #struct_ident::plain_size] {
                let mut ret:[u8; #struct_ident::plain_size] = [0; #struct_ident::plain_size];
                #(
                // PANIC-SAFETY: This won't be panic, since the ret's size is determined;
                ret.get_mut(fields::#fields_id::layout_range()).unwrap().copy_from_slice(&self.#fields_id.to_ne_bytes());
                )*
                ret
            }
        }

        impl#struct_ident {
            pub const plain_size: usize = #struct_size;

            pub fn flat<'a, End: Endianess<'a>>(raw: &'a [u8; #struct_ident::plain_size])->#struct_plain_name<'a, End> {
                #struct_plain_name::<'a, End>::raw_from(raw)
            }
        }
        
        mod flassor_field {
            use core::{
                convert::{AsRef, AsMut, TryInto, Into},
                borrow::Borrow
            };
            use crate::flassor::{ByteOrder, Endianess, Le, Be};
            use fields::#fields_trait_name;
            use super::#struct_ident;
            
            #[derive(Debug)]
            pub struct #struct_plain_name<'a, End: Endianess<'a>> {
                raw: &'a [u8; #struct_ident::plain_size],
                phantom: core::marker::PhantomData<End>
            }
            impl<'a, End: Endianess<'a>> #struct_plain_name<'a, End> {
                /// same as raw_from.
                pub fn map(raw:  &'a [u8; #struct_ident::plain_size])->Self {
                    Self {
                        raw,
                        phantom: core::marker::PhantomData
                    }
                }
                /// raw_from means we didn't check the internal value.
                pub fn raw_from(raw:  &'a [u8; #struct_ident::plain_size])->Self {
                    Self {
                        raw,
                        phantom: core::marker::PhantomData
                    }
                }
                pub fn raw(&self)-> &'a [u8; #struct_ident::plain_size] {
                    self.raw
                }
                pub fn get<T: fields::#fields_trait_name + ByteOrder<'a>>(&self)-> T {
                    // PANIC-SAFETY: This won't be panic, since the raw's size is determined.
                    End::from_bytes(self.raw.get(T::layout_range()).unwrap().try_into().unwrap())
                }
                pub fn to_meta(&'a self)-> #struct_ident {
                    #struct_ident {
                        #(
                            #fields_id: self.get::<fields::#fields_id>().raw(),
                        )*
                    }
                }
                pub unsafe fn as_meta(&'a self)-> &'a #struct_ident {
                    assert_eq!(core::mem::size_of::<#struct_ident>(), #struct_ident::plain_size, "The struct should be packed");

                    unsafe {
                        &*(self.raw.as_ptr() as *const #struct_ident)
                    }
                }
            }
            impl<'a, End: Endianess<'a>> AsRef<[u8]> for #struct_plain_name<'a, End> {
                fn as_ref(&self)->&[u8] {
                    self.raw
                }
            }
            #[derive(Debug)]
            pub struct #struct_plain_mut_name<'a, End: Endianess<'a>> {
                raw: &'a mut [u8; #struct_ident::plain_size],
                phantom: core::marker::PhantomData<End>
            }
            impl<'a, End: Endianess<'a>> #struct_plain_mut_name<'a, End> {
                pub fn new(raw:  &'a mut [u8; #struct_ident::plain_size])->Self {
                    Self {
                        raw,
                        phantom: core::marker::PhantomData
                    }
                }
                /// raw_from means we didn't check the internal value.
                pub fn raw_from(raw:  &'a mut [u8; #struct_ident::plain_size])->Self {
                    Self {
                        raw,
                        phantom: core::marker::PhantomData
                    }
                }
                pub fn raw_mut(&'a mut self)-> &'a mut [u8; #struct_ident::plain_size] {
                    self.raw
                }
                pub fn raw(&'a self)->&'a [u8; #struct_ident::plain_size] {
                    self.raw
                }
                fn as_mut(&mut self)->&mut [u8] {
                    self.raw
                }
                pub fn get<T: fields::#fields_trait_name + ByteOrder<'a>>(&'a self)-> T {
                    // PANIC-SAFETY: This won't be panic, since the raw's size is determined.
                    End::from_bytes(self.raw.get(T::layout_range()).unwrap().try_into().unwrap())
                }
                pub fn set<T: fields::#fields_trait_name + ByteOrder<'a>>(&'a mut self, value:T)-> &'a mut #struct_plain_mut_name<'a, End> {
                    self.raw[T::layout_range()].copy_from_slice(End::bytes_from(value).borrow());
                    self
                }
                pub fn to_meta(&'a self)-> #struct_ident {
                    #struct_ident {
                        #(
                            #fields_id: self.get::<fields::#fields_id>().raw(),
                        )*
                    }
                }
                pub unsafe fn as_meta(&'a self)-> &'a #struct_ident {
                    assert_eq!(core::mem::size_of::<#struct_ident>(), #struct_ident::plain_size, "The struct should be packed");

                    unsafe {
                        &*(self.raw.as_ptr() as *const #struct_ident)
                    }
                }
                pub unsafe fn as_mut_meta(&'a mut self)-> &'a mut #struct_ident {
                    assert_eq!(core::mem::size_of::<#struct_ident>(), #struct_ident::plain_size, "The struct should be packed");

                    unsafe {
                        &mut *(self.raw.as_mut_ptr() as *mut #struct_ident)
                    }
                }
            }
            impl<'a, End: Endianess<'a>> AsRef<[u8]> for #struct_plain_mut_name<'a, End> {
                fn as_ref(&self)->&[u8] {
                    self.raw
                }
            }
            pub mod fields {
                use core::convert::TryInto;
                use crate::flassor::ByteOrder;

                pub trait #fields_trait_name {
                    fn layout_range()->core::ops::Range<usize>;
                }
                #(
                    #[allow(non_camel_case_types)]
                    #[derive(Debug)]
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
                        pub fn new(value: #fields_ty)-> #fields_id {
                            #fields_id {
                                value
                            }
                        }
                        pub fn raw(&self)->#fields_ty {
                            self.value
                        }
                    }
                    impl<'a> ByteOrder<'a> for #fields_id {
                        type Bytes = [u8; core::mem::size_of::<#fields_ty>()];
                        fn to_ne_bytes(self) -> [u8; core::mem::size_of::<#fields_ty>()] {
                            #fields_ty::to_ne_bytes(self.value)
                        }
                        fn to_le_bytes(self) -> [u8; core::mem::size_of::<#fields_ty>()] {
                            #fields_ty::to_le_bytes(self.value)
                        }
                        fn to_be_bytes(self) -> [u8; core::mem::size_of::<#fields_ty>()] {
                            #fields_ty::to_be_bytes(self.value)
                        }
                        fn from_le_bytes(x: Self::Bytes) -> Self {
                            Self {value:#fields_ty::from_le_bytes(x)}
                        }
                        fn from_be_bytes(x: Self::Bytes) -> Self {
                            Self {value:#fields_ty::from_be_bytes(x)}
                        }
                        fn from_ne_bytes(x: Self::Bytes) -> Self {
                            Self {value:#fields_ty::from_ne_bytes(x)}
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
