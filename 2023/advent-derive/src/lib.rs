use std::{fs::read_dir, path::PathBuf};

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, punctuated::Punctuated, ItemFn, Token};

#[proc_macro]
pub fn years_enum(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let years_folder = PathBuf::from("advent-puzzles/src");
    if !years_folder.is_dir() {
        panic!("The years folder is not a directory");
    }
    let years = read_dir(years_folder)
        .expect("Failed to read years folder")
        .flat_map(|year| {
            let year = year.ok()?;
            if !year.path().is_dir() {
                return None;
            }
            let year = year.file_name().into_string().ok()?;
            let year = year.parse::<u16>().ok()?;
            Some(year)
        })
        .collect::<Vec<_>>();

    let year_idents = years
        .iter()
        .map(|year| format_ident!("Y{}", year))
        .collect::<Vec<_>>();

    let years_str = years
        .iter()
        .map(|year| year.to_string())
        .collect::<Vec<_>>();

    let tokens = quote::quote! {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, clap::ValueEnum)]
        enum Years {
            #(
                #[clap(name = #years_str)]
                #year_idents
            ),*
        }
        impl Into<u16> for Years {
            fn into(self) -> u16 {
                match self {
                    #(
                        Years::#year_idents => #years,
                    )*
                }
            }
        }
    };
    tokens.into()
}

#[proc_macro]
pub fn days_enum(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let days = 1u8..=25u8;

    let day_idents = days
        .clone()
        .map(|day| format_ident!("Day{}", day.to_string()))
        .collect::<Vec<_>>();

    let days_str = days.clone().map(|day| day.to_string()).collect::<Vec<_>>();

    let tokens = quote::quote! {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, clap::ValueEnum)]
        enum Days {
                #(
                    #[clap(name = #days_str)]
                    #day_idents,
                )*
                Today,
        }
        impl Into<u8> for Days {
            fn into(self) -> u8 {
                match self {
                    #(
                        Days::#day_idents => #days,
                    )*
                    Days::Today => todays_day(),
                }
            }
        }
    };
    tokens.into()
}

#[proc_macro]
pub fn year_declerations(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let entries = find_entries();

    let paths = entries.iter().map(|entry| {
        let Entry { year, day } = entry;
        format!("{}/day{}/mod.rs", year, day)
    });

    let identifiers = entries.iter().map(|entry| {
        let Entry { year, day } = entry;
        format_ident!("day{}_{}", day, year)
    });

    let tokens = quote::quote! {
        #(
            #[path = #paths]
            pub mod #identifiers;
        )*
    };
    tokens.into()
}

struct DayInvocationsInput {
    year: Ident,
    day: Ident,
    part: Ident,
    input: Ident,
}

impl syn::parse::Parse for DayInvocationsInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let year = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let day = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let part = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let input = input.parse()?;
        Ok(Self {
            year,
            day,
            part,
            input,
        })
    }
}

#[proc_macro]
pub fn day_invocations(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Get parameter identifiers for year: u16, day: u8, part: bool, input: &str
    let DayInvocationsInput {
        year,
        day,
        part,
        input,
    } = syn::parse_macro_input!(input as DayInvocationsInput);

    let entries = find_entries();
    let entry_years = entries.iter().map(|entry| entry.year).collect::<Vec<u16>>();
    let entry_days = entries.iter().map(|entry| entry.day).collect::<Vec<u8>>();

    let targets = entries.iter().map(|entry| {
        let Entry { year, day } = entry;
        format_ident!("day{}_{}", day, year)
    });

    let tokens = quote::quote! {
        match (#year, #day) {
            #(
                (#entry_years, #entry_days) => {
                    if #part {
                        #targets::part2(#input)
                    } else {
                        #targets::part1(#input)
                    }
                },
            )*
            _ => panic!("No solution found for year {} day {}", #year, #day),
        }
    };
    tokens.into()
}

#[derive(Debug)]
struct Entry {
    year: u16,
    day: u8,
}

fn find_entries() -> Vec<Entry> {
    let mut years_folder = PathBuf::from("advent-puzzles/src");

    if !years_folder.is_dir() {
        // When cargo expand is run from the root of the workspace, the years folder is in the root
        years_folder = PathBuf::from("src");
    }
    if !years_folder.is_dir() {
        panic!("The years folder is not a directory");
    }

    let years = read_dir(years_folder)
        .expect("Failed to read years folder")
        .flat_map(|year| {
            let year = year.expect("Failed to read year folder");
            if !year.path().is_dir() {
                return None;
            }
            let year_name = year
                .file_name()
                .into_string()
                .expect("Failed to read year name");
            let year_value = year_name.parse::<u16>().ok()?;
            let days = read_dir(year.path())
                .expect("Failed to read days folder")
                .flat_map(|day| {
                    let day = day.expect(
                        format!("Failed to read day folder in year {}", year_name).as_str(),
                    );
                    if !day.path().is_dir() {
                        return None;
                    }
                    let day = day
                        .file_name()
                        .into_string()
                        .expect(format!("Failed to read day name in year {}", year_name).as_str());
                    let day = day
                        .strip_prefix("day")
                        .expect(
                            format!("Failed to strip day prefix in year {}", year_name).as_str(),
                        )
                        .parse::<u8>()
                        .expect(
                            format!("Failed to parse day {} in year {}", day, year_name).as_str(),
                        );
                    Some(Entry {
                        year: year_value,
                        day,
                    })
                })
                .collect::<Vec<_>>();
            Some(days)
        })
        .flatten()
        .collect::<Vec<_>>();

    years
}

struct MemoizeArgs {
    key_args: Option<Vec<Ident>>,
}

impl syn::parse::Parse for MemoizeArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self { key_args: None });
        }

        let ident: Ident = input.parse()?;
        if ident != "key" {
            return Err(syn::Error::new(ident.span(), "expected `key`"));
        }

        input.parse::<Token![=]>()?;

        let content;
        syn::parenthesized!(content in input);
        let args: Punctuated<Ident, Token![,]> =
            content.parse_terminated(Ident::parse, Token![,])?;

        Ok(Self {
            key_args: Some(args.into_iter().collect()),
        })
    }
}

#[proc_macro_attribute]
pub fn memoize(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = parse_macro_input!(args as MemoizeArgs);
    let input_fn = parse_macro_input!(input as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();
    let fn_vis = &input_fn.vis;
    let fn_generics = &input_fn.sig.generics;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;
    let fn_body = &input_fn.block;
    let fn_attrs = &input_fn.attrs;

    let unmemoized_fn_name = format_ident!("{}_unmemoized", fn_name);
    let cache_mod_name = format_ident!("__{}_cache", fn_name);
    let cache_mod_pub_name = format_ident!("{}_cache", fn_name);

    let arg_names: Vec<_> = fn_inputs
        .iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    return Some(pat_ident.ident.clone());
                }
            }
            None
        })
        .collect();

    let arg_types: Vec<_> = fn_inputs
        .iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                return Some((*pat_type.ty).clone());
            }
            None
        })
        .collect();

    let key_args: Vec<_> = args.key_args.unwrap_or_else(|| arg_names.clone());

    let key_indices: Vec<usize> = key_args
        .iter()
        .map(|key_arg| {
            arg_names
                .iter()
                .position(|arg| arg == key_arg)
                .expect(&format!(
                    "key argument `{}` not found in function arguments",
                    key_arg
                ))
        })
        .collect();

    let key_types: Vec<_> = key_indices.iter().map(|&i| arg_types[i].clone()).collect();

    let key_clone_exprs: Vec<TokenStream> = key_args
        .iter()
        .zip(key_types.iter())
        .map(|(name, ty)| {
            let ty_str = quote!(#ty).to_string();
            if ty_str.starts_with("&") {
                quote! { #name.to_owned() }
            } else {
                quote! { #name.clone() }
            }
        })
        .collect();

    let return_type = match fn_output {
        syn::ReturnType::Default => quote! { () },
        syn::ReturnType::Type(_, ty) => quote! { #ty },
    };

    let owned_key_types: Vec<TokenStream> = key_types
        .iter()
        .map(|ty| {
            let ty_str = quote!(#ty).to_string();
            if ty_str.starts_with("& '") || ty_str.starts_with("&'") {
                let inner = ty_str
                    .split_once(char::is_whitespace)
                    .map(|(_, rest)| rest)
                    .unwrap_or(&ty_str[1..]);
                let inner_tokens: TokenStream = inner.parse().unwrap_or_else(|_| quote! { #ty });
                quote! { <#inner_tokens as ToOwned>::Owned }
            } else if ty_str.starts_with("&") {
                let inner = &ty_str[1..].trim();
                let inner_tokens: TokenStream = inner.parse().unwrap_or_else(|_| quote! { #ty });
                quote! { <#inner_tokens as ToOwned>::Owned }
            } else {
                quote! { #ty }
            }
        })
        .collect();

    let key_type = if owned_key_types.len() == 1 {
        let t = &owned_key_types[0];
        quote! { #t }
    } else {
        quote! { (#(#owned_key_types),*) }
    };

    let key_expr = if key_args.len() == 1 {
        let clone_expr = &key_clone_exprs[0];
        quote! { #clone_expr }
    } else {
        quote! { (#(#key_clone_exprs),*) }
    };

    let output = quote! {
        #[allow(unused)]
        #(#fn_attrs)*
        #fn_vis fn #fn_name #fn_generics(#fn_inputs) #fn_output {
            let __key = #key_expr;

            {
                let cache = #cache_mod_name::CACHE.read().unwrap();
                if let Some(result) = cache.get(&__key) {
                    #cache_mod_name::STATS.record_hit();
                    return result.clone();
                }
            }

            #cache_mod_name::STATS.record_miss();
            let __result = #unmemoized_fn_name(#(#arg_names),*);

            {
                let mut cache = #cache_mod_name::CACHE.write().unwrap();
                cache.insert(__key, __result.clone());
            }

            __result
        }

        #[allow(dead_code)]
        #(#fn_attrs)*
        #fn_vis fn #unmemoized_fn_name #fn_generics(#fn_inputs) #fn_output #fn_body

        #[doc(hidden)]
        #fn_vis mod #cache_mod_name {
            use super::*;
            use std::sync::{RwLock, LazyLock};
            use std::collections::HashMap;

            pub static CACHE: LazyLock<RwLock<HashMap<#key_type, #return_type>>> = LazyLock::new(|| RwLock::new(HashMap::new()));
            pub static STATS: AtomicStats = AtomicStats::new();

            pub fn clear() {
                let mut cache = CACHE.write().unwrap();
                cache.clear();
            }

            pub fn invalidate(key: &#key_type) {
                let mut cache = CACHE.write().unwrap();
                cache.remove(key);
            }

            pub fn stats() -> MemoizeStats {
                let cache = CACHE.read().unwrap();
                MemoizeStats {
                    function_name: #fn_name_str,
                    hits: STATS.get_hits(),
                    misses: STATS.get_misses(),
                    cache_size: cache.len(),
                }
            }

            pub fn reset_stats() {
                STATS.reset();
            }

            fn __get_stats() -> MemoizeStats {
                stats()
            }

            fn __clear_cache() {
                clear();
            }

            fn __reset_stats() {
                reset_stats();
            }

            inventory::submit! {
                MemoizeStatsProvider {
                    name: #fn_name_str,
                    get_stats: __get_stats,
                    clear_cache: __clear_cache,
                    reset_stats: __reset_stats,
                }
            }
        }

        #fn_vis use #cache_mod_name as #cache_mod_pub_name;
    };

    output.into()
}
