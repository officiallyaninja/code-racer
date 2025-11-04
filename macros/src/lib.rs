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
                "ChallengeTypes only supports structs with named fields",
            )
            .to_compile_error()
            .into();
        }
    };

    // Collect all field names except `data_version`
    let variants = fields.iter().filter_map(|f| {
        let name = f.ident.as_ref()?;
        if name == "data_version" {
            return None;
        }

        // Capitalize the first letter for enum variant
        let mut variant_name = name.to_string();
        variant_name.replace_range(0..1, &variant_name[0..1].to_uppercase());
        Some(Ident::new(&variant_name, name.span()))
    });

    // Generate the enum
    let enum_def = quote! {
        pub enum ChallengeType {
            #(#variants),*
        }
    };

    // Keep the original struct + add the enum
    let expanded = quote! {
        #input
        #enum_def
    };

    expanded.into()
}
