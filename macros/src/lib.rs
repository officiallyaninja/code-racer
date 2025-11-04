use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FieldsNamed, Ident, ItemStruct};

#[proc_macro_attribute]
pub fn generate_challenge_types(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input struct
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;

    // Extract named fields
    let fields = match &input.fields {
        syn::Fields::Named(FieldsNamed { named, .. }) => named,
        _ => {
            return syn::Error::new_spanned(
                struct_name,
                "generate_challenge_types only supports structs with named fields",
            )
            .to_compile_error()
            .into();
        }
    };

    // Build enum variants
    let variants: Vec<Ident> = fields
        .iter()
        .filter_map(|f| {
            let name = f.ident.as_ref()?;
            if name == "data_version" {
                return None;
            }

            let mut variant_name = name.to_string();
            variant_name.replace_range(0..1, &variant_name[0..1].to_uppercase());
            Some(Ident::new(&variant_name, name.span()))
        })
        .collect();

    // Build match arms for TryFrom<&str>
    let variant_arms = variants.iter().map(|variant_ident| {
        let variant_str = variant_ident.to_string().to_lowercase();
        quote! {
            #variant_str => Ok(ChallengeType::#variant_ident),
        }
    });

    // Generate the enum definition
    let enum_def = quote! {
        #[derive(Debug)]
        pub enum ChallengeType {
            #(#variants),*
        }

        impl TryFrom<&str> for ChallengeType {
            type Error = ();

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value.to_lowercase().as_ref() {
                    #(#variant_arms)*
                    _ => Err(()),
                }
            }
        }
    };

    // Combine the original struct + generated enum
    let expanded = quote! {
        #input
        #enum_def
    };

    expanded.into()
}
