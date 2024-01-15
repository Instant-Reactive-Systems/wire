use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{parse_macro_input, spanned::Spanned, token::Pub, Data, DeriveInput, Error};
use quote::*;

/// Derives a struct for each enum variant, with the same name as the variant.
///
/// # Example
/// ```
/// #[derive(wire::WireObj)]
/// #[rustfmt::ignore] // rustfmt will mess up the generated code
/// #[derive(Debug, Clone, PartialEq)]
/// pub enum MyEnum {
/// 	Foo { a: u32, b: u32 },
/// 	Bar(u32, u32),
/// }
/// ```
#[proc_macro_derive(WireObj)]
pub fn derive_wire_obj(input: TokenStream) -> TokenStream {
	let mut input = parse_macro_input!(input as DeriveInput);

	let data = match input.data {
		Data::Enum(ref mut data) => data,
		_ => return Error::new(input.ident.span(), "wire only works on enums").into_compile_error().into(),
	};

	if input.generics.lt_token.is_some() {
		return Error::new(input.generics.span(), "wire does not support generics").into_compile_error().into()
	}

	let variant_structs = data
		.variants
		.iter()
		.map(|v| {
			let name = v.ident.clone();
			match &v.fields {
				syn::Fields::Named(..) => {
					let fields = v
						.fields
						.iter()
						.cloned()
						.map(|field| syn::Field {
							vis: syn::Visibility::Public(syn::VisPublic { pub_token: Pub(Span::call_site()) }),
							..field
						})
						.collect::<Vec<_>>();
					let attrs = &input.attrs;
					quote! {
						#(#attrs)*
						pub struct #name {
							#(#fields),*
						}
					}
				},
				syn::Fields::Unnamed(..) => {
					let fields = v
						.fields
						.iter()
						.cloned()
						.map(|field| syn::Field {
							vis: syn::Visibility::Public(syn::VisPublic { pub_token: Pub(Span::call_site()) }),
							..field
						})
						.collect::<Vec<_>>();
					let attrs = &input.attrs;
					quote! {
						#(#attrs)*
						pub struct #name (#(#fields),*);
					}
				},
				syn::Fields::Unit => {
					let attrs = &input.attrs;
					quote! {
						#(#attrs)*
						pub struct #name;
					}
				},
			}
		})
		.collect::<Vec<_>>();

	let res = quote! {
		#(#variant_structs)*
	};

	// use std::io::Write;
	// let mut path = std::path::PathBuf::from("target");
	// path.push("generated");
	// path.push("wire");
	// std::fs::create_dir_all(&path).expect("unable to create export directory");
	// path.push("generated.rs");
	// let mut file = std::fs::File::create(path).expect("unable to create export file");
	// file.write_all(res.to_string().as_bytes()).expect("unable to write export file");

	res.into()
}

// todo: switch to virtue once it gets attributes on structs
//
// use virtue::{prelude::*, generate::Parent};
// use virtue::generate::StreamBuilder;
//
// #[proc_macro_derive(WireObj)]
// pub fn derive_wire_obj(input: TokenStream) -> TokenStream {
// derive_wire_obj_inner(input).unwrap_or_else(|error| error.into_token_stream())
// }
//
// fn derive_wire_obj_inner(input: TokenStream) -> Result<TokenStream> {
// Parse the struct or enum you want to implement a derive for
// let parse = Parse::new(input)?;
// Get a reference to the generator
// let (mut generator, attrs, body) = parse.into_generator();
// let Body::Enum(body) = body else {
// return Err(Error::Custom {
// error: "WireObj does not support structs, only enums".into(),
// span: None,
// })
// };
//
// {
// let mut m = generator.generate_mod("ecs");
// body.variants.iter().for_each(|var| {
// let Some(Fields::Struct(fields)) = &var.fields else { return };
// let mut s = m.generate_struct(var.name.to_string());
// s.make_pub();
//
// fields.iter().for_each(|field| {
// s.add_pub_field(field.0.to_string(), field.1.type_string());
// });
// });
// }
//
// generator.finish()
// }
