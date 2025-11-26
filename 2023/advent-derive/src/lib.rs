use std::{fs::read_dir, path::PathBuf};

use proc_macro2::Ident;
use quote::format_ident;

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
