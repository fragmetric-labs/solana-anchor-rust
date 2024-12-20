use std::collections::BTreeMap;

use anchor_lang_idl_spec::{IdlAccount, IdlDefinedFields, IdlField, IdlInstructionAccountItem, IdlTypeDef, IdlTypeDefTy};
use heck::{ToPascalCase, ToSnakeCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{generate_fields, get_field_list_properties, StructOpts};

/// Generates a list of [IdlInstructionAccountItem]s as a [TokenStream].
pub fn generate_account_fields(
    name: &str,
    accounts: &[IdlInstructionAccountItem],
) -> (TokenStream, TokenStream) {
    let mut all_structs: Vec<TokenStream> = vec![];
    let all_fields = accounts
        .iter()
        .map(|account| match account {
            IdlInstructionAccountItem::Single(info) => {
                let acc_name = format_ident!("{}", info.name.to_snake_case());
                let annotation = if info.writable {
                    quote! { #[account(mut)] }
                } else {
                    quote! {}
                };
                let ty = if info.signer {
                    quote! { Signer<'info> }
                } else {
                    quote! { AccountInfo<'info> }
                };
                quote! {
                   #annotation
                   pub #acc_name: #ty
                }
            }
            IdlInstructionAccountItem::Composite(inner) => {
                let field_name = format_ident!("{}{}", name, inner.name.to_snake_case());
                let sub_name = format!("{}{}", name, inner.name.to_pascal_case());
                let sub_ident = format_ident!("{}", &sub_name);
                let (sub_structs, sub_fields) = generate_account_fields(&sub_name, &inner.accounts);
                all_structs.push(sub_structs);
                all_structs.push(quote! {
                    #[derive(Accounts)]
                    pub struct #sub_ident<'info> {
                        #sub_fields
                    }
                });
                quote! {
                    pub #field_name: #sub_ident<'info>
                }
            }
        })
        .collect::<Vec<_>>();
    (
        quote! {
            #(#all_structs)*
        },
        quote! {
            #(#all_fields),*
        },
    )
}


/// Generates an account state struct.
pub fn generate_account(
    defs: &[IdlTypeDef],
    account_name: &str,
    fields: &[IdlField],
    opts: StructOpts,
) -> TokenStream {
    let props = get_field_list_properties(defs, fields);

    let derive_copy = if props.can_copy && !opts.zero_copy {
        quote! {
            #[derive(Copy)]
        }
    } else {
        quote! {}
    };
    let derive_default = if props.can_derive_default && false {
        quote! {
            #[derive(Default)]
        }
    } else {
        quote! {}
    };
    let derive_account = if opts.zero_copy {
        let repr = if opts.packed {
            quote! {
                #[repr(packed)]
            }
        } else {
            quote! {
                #[repr(C)]
            }
        };
        quote! {
            #[account(zero_copy)]
            #repr
            #[derive(Debug)]
        }
    } else {
        quote! {
            #[account]
            #[derive(Debug)]
        }
    };

    let doc = format!(" Account: {}", account_name);
    let struct_name = format_ident!("{}", account_name);
    let fields_rendered = generate_fields(fields);
    quote! {
        #derive_account
        #[doc = #doc]
        #derive_copy
        #derive_default
        pub struct #struct_name {
            #fields_rendered
        }
    }
}

/// Generates account state structs.
pub fn generate_accounts(
    typedefs: &[IdlTypeDef],
    accounts: &[IdlAccount],
    struct_opts: &BTreeMap<String, StructOpts>,
) -> TokenStream {
    let defined = accounts.iter().map(|account| typedefs.iter()
        .find(|type_def| type_def.name == account.name).unwrap())
        .map(|account_type_def| match &account_type_def.ty {
        anchor_lang_idl_spec::IdlTypeDefTy::Struct { fields } => {
            let opts = struct_opts.get(&account_type_def.name).copied().unwrap_or_default();
            let fields = fields.iter().flat_map(|f| {
                match f {
                    IdlDefinedFields::Named(ff) => {
                        ff.clone()
                    }
                    _ => todo!(),
                }
            }).collect::<Vec<_>>();
            generate_account(typedefs, &account_type_def.name, &*fields, opts)
        }
        IdlTypeDefTy::Enum { .. } => {
            todo!()
        }
        IdlTypeDefTy::Type { .. } => {
            todo!()
        }
    });
    quote! {
        #(#defined)*
    }
}
