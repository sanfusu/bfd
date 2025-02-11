extern crate case;
use case::CaseExt;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn accessor(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let layout = gen_accessor(input);
    layout.into()
}

fn gen_accessor(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let ast_struct = ast.clone();
    // if ast
    //     .attrs
    //     .iter()
    //     .find(|&x| x.to_token_stream().to_string() == "#[repr(C)]")
    //     .is_none()
    // {
    //     return syn::parse::Error::new_spanned(
    //         ast.ident.to_token_stream(),
    //         "The struct should be a repr(c)",
    //     )
    //     .to_compile_error();
    // }
    let struct_ident = ast.ident;
    let fields_trait_name = format_ident!("{}Fields", struct_ident.to_string());
    let struct_plain_name = format_ident!("{}Flat", struct_ident.to_string());
    let struct_plain_mut_name = format_ident!("{}Mut", struct_plain_name);
    let accessor_mod_name = format_ident!("{}_accessor", struct_ident.to_string().to_snake());
    let mut fields_id = Vec::<syn::Ident>::new();
    let mut fields_id_camel = Vec::<syn::Ident>::new();
    let mut fields_ty = Vec::<syn::Type>::new();
    if let syn::Data::Struct(data) = ast.data {
        data.fields.iter().for_each(|x| {
            fields_id_camel.push(format_ident!(
                "{}",
                x.ident.to_owned().unwrap().to_string().to_camel()
            ));
            fields_id.push(x.ident.to_owned().unwrap());
            fields_ty.push(x.ty.to_owned());
        });
    }
    let mut fields_range = Vec::<proc_macro2::TokenStream>::new();
    fields_id[0..]
        .iter()
        .zip(fields_ty[0..].iter())
        .for_each(|(id,ty)| {
            fields_range.push(quote!(
                unsafe {
                    (&(*(0 as *const #struct_ident)).#id as *const #ty as *const u8 as usize)
                    ..
                    (&(*(0 as *const #struct_ident)).#id as *const #ty as *const u8 as usize) + core::mem::size_of::<#ty>()})
            )
        });

    let (struct_ident_as_slice_fn, struct_plain_as_meta, struct_plain_mut_as_meta) = (
        Some(quote! {
            pub fn as_slice<'a>(&'a self)->&'a [u8] {
                unsafe {
                    core::slice::from_raw_parts(self as * const #struct_ident as * const u8, <#struct_ident>::flat_size())
                }
            }
        }),
        Some(quote! {
            /// 需要确保起始地址对齐。
            /// rust 中对未对齐地址的解引用是 undefined 行为。
            pub fn as_meta(&'a self)-> Result<&'a #struct_ident, ()>{
                unsafe {
                    if self.raw.as_ptr() as usize % core::mem::align_of::<#struct_ident>() == 0 {
                        Ok(&*(self.raw.as_ptr() as *const #struct_ident))
                    } else {
                        Err(())
                    }
                }
            }
            /// 不检查地址对齐
            pub unsafe fn as_meta_uncheck(&'a self)-> &'a #struct_ident {
                &*(self.raw.as_ptr() as *const #struct_ident)
            }
        }),
        Some(quote! {
            /// 除了可修改之外，等同 as_meta
            pub fn as_mut_meta(&'a mut self)-> Result<&'a mut #struct_ident, ()> {
                unsafe {
                    if self.raw.as_ptr() as usize % core::mem::align_of::<#struct_ident>() == 0 {
                        Ok(&mut *(self.raw.as_mut_ptr() as *mut #struct_ident))
                    } else {
                        Err(())
                    }
                }
            }
            /// 不检查地址对齐
            pub unsafe fn as_mut_meta_uncheck(&'a mut self)-> &'a mut #struct_ident {
                &mut *(self.raw.as_mut_ptr() as *mut #struct_ident)
            }
        }),
    );

    quote! {
        #[repr(C)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
        #ast_struct
        impl Into<[u8; <#struct_ident>::flat_size()]> for #struct_ident {
            fn into(self)->[u8; <#struct_ident>::flat_size()] {
                let mut ret:[u8; <#struct_ident>::flat_size()] = [0; <#struct_ident>::flat_size()];
                #(
                // PANIC-SAFETY: This won't be panic, since the ret's size is determined;
                ret.get_mut(<#accessor_mod_name::fields::#fields_id_camel as #accessor_mod_name::fields::#fields_trait_name>::layout_range()).unwrap().copy_from_slice(&self.#fields_id.to_ne_bytes());
                )*
                ret
            }
        }
        impl #struct_ident {
            pub const fn flat_size()-> usize{
                core::mem::size_of::<#struct_ident>()
            }


            pub fn flat<'a, End: crate::flassor::Endianess<'a>>(raw: &'a [u8; <#struct_ident>::flat_size()])->#accessor_mod_name::#struct_plain_name<'a, End> {
                #accessor_mod_name::#struct_plain_name::<'a, End>::from_raw(raw)
            }

            pub fn flat_mut<'a, End: crate::flassor::Endianess<'a>>(raw: &'a mut [u8; <#struct_ident>::flat_size()])->#accessor_mod_name::#struct_plain_mut_name<'a, End> {
                #accessor_mod_name::#struct_plain_mut_name::<'a, End>::from_raw(raw)
            }

            #struct_ident_as_slice_fn
        }
        impl<'a> flassor::ByteOrder<'a> for #struct_ident {
            type Bytes = [u8; <#struct_ident>::flat_size()];
            fn to_ne_bytes(self) -> [u8; <#struct_ident>::flat_size()] {
                self.into()
            }
            fn to_le_bytes(self) -> [u8; <#struct_ident>::flat_size()] {
                let ret = #struct_ident {
                    #(#fields_id: <#fields_ty>::to_le(self.#fields_id)),*
                };
                ret.into()
            }
            fn to_be_bytes(self) -> [u8; <#struct_ident>::flat_size()] {
                let ret = #struct_ident {
                    #(#fields_id: <#fields_ty>::to_be(self.#fields_id)),*
                };
                ret.into()
            }
            fn from_le_bytes(x: Self::Bytes) -> Self {
                let ret = <#struct_ident>::flat::<flassor::Le>(&x);
                ret.to_meta()
            }
            fn from_be_bytes(x: Self::Bytes) -> Self {
                let ret = <#struct_ident>::flat::<flassor::Be>(&x);
                ret.to_meta()
            }
            fn from_ne_bytes(x: Self::Bytes) -> Self {
                let ret = <#struct_ident>::flat::<flassor::Ne>(&x);
                ret.to_meta()
            }
            fn from_be(x: Self) -> Self {
                Self {
                    #(#fields_id: <#fields_ty>::from_be(x.#fields_id)),*
                }
            }
            fn from_le(x: Self) -> Self {
                Self {
                    #(#fields_id: <#fields_ty>::from_le(x.#fields_id)),*
                }
            }
            fn to_be(self) -> Self {
                Self {
                    #(#fields_id: <#fields_ty>::to_be(self.#fields_id)),*
                }
            }
            fn to_le(self) -> Self {
                Self {
                    #(#fields_id: <#fields_ty>::to_le(self.#fields_id)),*
                }
            }
        }

        mod #accessor_mod_name {
            use super::#struct_ident;
            use core::{
                convert::{AsRef, AsMut, TryInto, Into},
                borrow::Borrow
            };
            use flassor::{ByteOrder, Endianess, Le, Be};
            use fields::#fields_trait_name;

            #[derive(Debug)]
            pub struct #struct_plain_name<'a, End: Endianess<'a>> {
                raw: &'a [u8; <#struct_ident>::flat_size()],
                phantom: core::marker::PhantomData<End>
            }
            impl<'a, End: Endianess<'a>> #struct_plain_name<'a, End> {
                /// raw_from means we didn't check the internal value.
                pub fn from_raw(raw:  &'a [u8; <#struct_ident>::flat_size()])->Self {
                    Self {
                        raw,
                        phantom: core::marker::PhantomData
                    }
                }
                pub fn raw(&self)-> &'a [u8; <#struct_ident>::flat_size()] {
                    self.raw
                }
                pub fn get<T: fields::#fields_trait_name<'a>>(&self)-> T {
                    // PANIC-SAFETY: This won't be panic, since the raw's size is determined.
                    End::from_bytes(self.raw.get(T::layout_range()).unwrap().try_into().unwrap())
                }
                pub fn to_meta(&'a self)-> #struct_ident {
                    #struct_ident {
                        #(
                            #fields_id: self.get::<fields::#fields_id_camel>().raw(),
                        )*
                    }
                }
                #struct_plain_as_meta
            }
            impl<'a, End: Endianess<'a>> AsRef<[u8]> for #struct_plain_name<'a, End> {
                fn as_ref(&self)->&[u8] {
                    self.raw
                }
            }
            #[derive(Debug)]
            pub struct #struct_plain_mut_name<'a, End: Endianess<'a>> {
                raw: &'a mut [u8; <#struct_ident>::flat_size()],
                phantom: core::marker::PhantomData<End>
            }
            impl<'a, End: Endianess<'a>> #struct_plain_mut_name<'a, End> {
                pub fn new(raw:  &'a mut [u8; <#struct_ident>::flat_size()])->Self {
                    Self {
                        raw,
                        phantom: core::marker::PhantomData
                    }
                }
                /// from_raw means we didn't check the internal value.
                pub fn from_raw(raw:  &'a mut [u8; <#struct_ident>::flat_size()])->Self {
                    Self {
                        raw,
                        phantom: core::marker::PhantomData
                    }
                }
                pub fn raw_mut(&'a mut self)-> &'a mut [u8; <#struct_ident>::flat_size()] {
                    self.raw
                }
                pub fn raw(&'a self)->&'a [u8; <#struct_ident>::flat_size()] {
                    self.raw
                }
                fn as_mut(&mut self)->&mut [u8] {
                    self.raw
                }
                pub fn get<T: fields::#fields_trait_name<'a>>(&'a self)-> T {
                    // PANIC-SAFETY: This won't be panic, since the raw's size is determined.
                    End::from_bytes(self.raw.get(T::layout_range()).unwrap().try_into().unwrap())
                }
                pub fn set<T: fields::#fields_trait_name<'a>>(&'a mut self, value:T)-> &'a mut #struct_plain_mut_name<'a, End> {
                    self.raw[T::layout_range()].copy_from_slice(End::bytes_from(value).borrow());
                    self
                }
                pub fn to_meta(&'a self)-> #struct_ident {
                    #struct_ident {
                        #(
                            #fields_id: self.get::<fields::#fields_id_camel>().raw(),
                        )*
                    }
                }
                pub fn to_flat(&'a self)->#struct_plain_name<'a, End> {
                    #struct_plain_name::from_raw(self.raw())
                }
               #struct_plain_as_meta
               #struct_plain_mut_as_meta
            }
            impl<'a, End: Endianess<'a>> AsRef<[u8]> for #struct_plain_mut_name<'a, End> {
                fn as_ref(&self)->&[u8] {
                    self.raw
                }
            }
            pub mod fields {
                use super::super::*;
                use flassor::ByteOrder;

                pub trait #fields_trait_name<'a>: ByteOrder<'a> {
                    fn layout_range()->core::ops::Range<usize>;
                }
                #(
                    #[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
                    pub struct #fields_id_camel {
                        value: #fields_ty
                    }
                    impl<'a> #fields_trait_name<'a> for #fields_id_camel{
                        #[inline]
                        fn layout_range()->core::ops::Range<usize> {
                            #fields_range
                        }
                    }
                    impl #fields_id_camel {
                        /// new 函数用于创建性的字段值，但其可见域为 crate，如果需要暴露成接口，可以自行实现 convert::from trait。
                        pub(crate) const fn new(value: #fields_ty)-> #fields_id_camel {
                            #fields_id_camel {
                                value
                            }
                        }
                        pub const fn raw(&self)->#fields_ty {
                            self.value
                        }
                    }
                    impl<'a> ByteOrder<'a> for #fields_id_camel {
                        type Bytes = [u8; core::mem::size_of::<#fields_ty>()];
                        fn to_ne_bytes(self) -> [u8; core::mem::size_of::<#fields_ty>()] {
                            <#fields_ty>::to_ne_bytes(self.value)
                        }
                        fn to_le_bytes(self) -> [u8; core::mem::size_of::<#fields_ty>()] {
                            <#fields_ty>::to_le_bytes(self.value)
                        }
                        fn to_be_bytes(self) -> [u8; core::mem::size_of::<#fields_ty>()] {
                            <#fields_ty>::to_be_bytes(self.value)
                        }
                        fn from_le_bytes(x: Self::Bytes) -> Self {
                            Self {value:<#fields_ty>::from_le_bytes(x)}
                        }
                        fn from_be_bytes(x: Self::Bytes) -> Self {
                            Self {value:<#fields_ty>::from_be_bytes(x)}
                        }
                        fn from_ne_bytes(x: Self::Bytes) -> Self {
                            Self {value:<#fields_ty>::from_ne_bytes(x)}
                        }
                        fn from_be(x: Self) -> Self {
                            Self {
                                value: <#fields_ty>::from_be(x.value)
                            }
                        }
                        fn from_le(x: Self) -> Self {
                            Self {
                                value: <#fields_ty>::from_le(x.value)
                            }
                        }
                        fn to_be(self) -> Self {
                            Self {
                                value: <#fields_ty>::to_be(self.value)
                            }
                        }
                        fn to_le(self) -> Self {
                            Self {
                                value: <#fields_ty>::to_le(self.value)
                            }
                        }
                    }
                )*
            }
        }
    }
}
