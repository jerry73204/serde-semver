use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned as _,
    Token,
};

struct UnitStruct {
    #[allow(dead_code)]
    vis: syn::Visibility,
    attrs: Vec<syn::Attribute>,
    #[allow(dead_code)]
    struct_token: Token![struct],
    name: syn::Ident,
    #[allow(dead_code)]
    semi_token: Token![;],
}

impl Parse for UnitStruct {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(UnitStruct {
            attrs: input.call(syn::Attribute::parse_outer)?,
            vis: input.parse()?,
            struct_token: input.parse()?,
            name: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

#[proc_macro_derive(SemverReq, attributes(version))]
pub fn derive_semver_req(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as UnitStruct);
    f_derive_semver_req(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn f_derive_semver_req(input: UnitStruct) -> Result<proc_macro2::TokenStream, syn::Error> {
    let UnitStruct { attrs, name, .. } = input;

    let mut attrs_iter = attrs.iter().filter_map(|attr| {
        let ident = attr.path.get_ident()?;
        (ident == "version").then(|| attr)
    });

    let version_attr = attrs_iter.next().ok_or_else(|| {
        syn::Error::new(
            name.span(),
            r#"#[version("X.Y.Z")] attribute must be specified"#,
        )
    })?;

    if let Some(dup_attr) = attrs_iter.next() {
        return Err(syn::Error::new(
            dup_attr.span(),
            r#"duplicated version attribute"#,
        ));
    }

    let version_lit: syn::LitStr = version_attr.parse_args()?;
    let version_text = version_lit.value();
    semver::Version::parse(&version_text)
        .map_err(|err| syn::Error::new(version_lit.span(), err.to_string()))?;

    let expanded = quote! {
        impl #name {
            pub fn version() -> ::serde_semver::semver::Version {
                ::serde_semver::semver::Version::parse(#version_text).unwrap()
            }
        }

        impl ::serde_semver::serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde_semver::serde::Serializer,
            {
                use ::serde_semver::semver::{Version, VersionReq};
                use ::serde_semver::serde::de::Error;
                Self::version().serialize(serializer)
            }
        }

        impl<'de> ::serde_semver::serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde_semver::serde::Deserializer<'de>,
            {
                use ::serde_semver::semver::{Version, VersionReq};
                use ::serde_semver::serde::de::Error;
                let input_version = Version::deserialize(deserializer)?;
                let req = VersionReq::parse(&input_version.to_string()).unwrap();
                let target_version = Self::version();

                if !req.matches(&target_version) {
                    return Err(D::Error::custom(
                        format!(r#"input version {} is not compatible with version {}"#, input_version, target_version)
                    ));
                }

                Ok(Self)
            }
        }
    };

    Ok(expanded)
}
