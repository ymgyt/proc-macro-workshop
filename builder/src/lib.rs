extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // eprintln!("{:#?}", input);

    let name = input.ident;
    let builder_ident = syn::Ident::new(&format!("{}Builder", name), name.span());

    let quoted = quote! {
        pub struct #builder_ident {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        impl #name {
            fn builder() -> #builder_ident {
                #builder_ident {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }
    };

    proc_macro::TokenStream::from(quoted)
}

/*
DeriveInput {
    attrs: [],
    vis: Public(
        VisPublic {
            pub_token: Pub,
        },
    ),
    ident: Ident {
        ident: "Command",
        span: #0 bytes(1374..1381),
    },
    generics: Generics {
        lt_token: None,
        params: [],
        gt_token: None,
        where_clause: None,
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Named(
                FieldsNamed {
                    brace_token: Brace,
                    named: [
                        Field {
                            attrs: [],
                            vis: Inherited,
                            ident: Some(
                                Ident {
                                    ident: "executable",
                                    span: #0 bytes(1388..1398),
                                },
                            ),
                            colon_token: Some(
                                Colon,
                            ),
                            ty: Path(
                                TypePath {
                                    qself: None,
                                    path: Path {
                                        leading_colon: None,
                                        segments: [
                                            PathSegment {
                                                ident: Ident {
                                                    ident: "String",
                                                    span: #0 bytes(1400..1406),
                                                },
                                                arguments: None,
                                            },
                                        ],
                                    },
                                },
                            ),
                        },
                        Comma,
                        Field {
                            attrs: [],
                            vis: Inherited,
                            ident: Some(
                                Ident {
                                    ident: "args",
                                    span: #0 bytes(1412..1416),
                                },
                            ),
                            colon_token: Some(
                                Colon,
                            ),
                            ty: Path(
                                TypePath {
                                    qself: None,
                                    path: Path {
                                        leading_colon: None,
                                        segments: [
                                            PathSegment {
                                                ident: Ident {
                                                    ident: "Vec",
                                                    span: #0 bytes(1418..1421),
                                                },
                                                arguments: AngleBracketed(
                                                    AngleBracketedGenericArguments {
                                                        colon2_token: None,
                                                        lt_token: Lt,
                                                        args: [
                                                            Type(
                                                                Path(
                                                                    TypePath {
                                                                        qself: None,
                                                                        path: Path {
                                                                            leading_colon: None,
                                                                            segments: [
                                                                                PathSegment {
                                                                                    ident: Ident {
                                                                                        ident: "String",
                                                                                        span: #0 bytes(1422..1428),
                                                                                    },
                                                                                    arguments: None,
                                                                                },
                                                                            ],
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        gt_token: Gt,
                                                    },
                                                ),
                                            },
                                        ],
                                    },
                                },
                            ),
                        },
                        Comma,
                        Field {
                            attrs: [],
                            vis: Inherited,
                            ident: Some(
                                Ident {
                                    ident: "env",
                                    span: #0 bytes(1435..1438),
                                },
                            ),
                            colon_token: Some(
                                Colon,
                            ),
                            ty: Path(
                                TypePath {
                                    qself: None,
                                    path: Path {
                                        leading_colon: None,
                                        segments: [
                                            PathSegment {
                                                ident: Ident {
                                                    ident: "Vec",
                                                    span: #0 bytes(1440..1443),
                                                },
                                                arguments: AngleBracketed(
                                                    AngleBracketedGenericArguments {
                                                        colon2_token: None,
                                                        lt_token: Lt,
                                                        args: [
                                                            Type(
                                                                Path(
                                                                    TypePath {
                                                                        qself: None,
                                                                        path: Path {
                                                                            leading_colon: None,
                                                                            segments: [
                                                                                PathSegment {
                                                                                    ident: Ident {
                                                                                        ident: "String",
                                                                                        span: #0 bytes(1444..1450),
                                                                                    },
                                                                                    arguments: None,
                                                                                },
                                                                            ],
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        gt_token: Gt,
                                                    },
                                                ),
                                            },
                                        ],
                                    },
                                },
                            ),
                        },
                        Comma,
                        Field {
                            attrs: [],
                            vis: Inherited,
                            ident: Some(
                                Ident {
                                    ident: "current_dir",
                                    span: #0 bytes(1457..1468),
                                },
                            ),
                            colon_token: Some(
                                Colon,
                            ),
                            ty: Path(
                                TypePath {
                                    qself: None,
                                    path: Path {
                                        leading_colon: None,
                                        segments: [
                                            PathSegment {
                                                ident: Ident {
                                                    ident: "String",
                                                    span: #0 bytes(1470..1476),
                                                },
                                                arguments: None,
                                            },
                                        ],
                                    },
                                },
                            ),
                        },
                        Comma,
                    ],
                },
            ),
            semi_token: None,
        },
    ),
}
*/