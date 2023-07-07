use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn parse_logos(_input: TokenStream) -> TokenStream {
    let raw_logos = include_str!("../logos.sh").replace("\r\n", "\n");
    let raw_logos = raw_logos
        .split_once("in\n")
        .expect("Invalid logos.sh file")
        .1;
    let raw_logos = raw_logos
        .split_once("\nesac")
        .expect("Invalid logos.sh file")
        .0;

    let mut tux = None;
    let logos = raw_logos
        .split(";;\n")
        .filter_map(|raw_logo| {
            let (is_tux, logo) = pfetch_logo_parser::parse_logo(raw_logo)?;
            if is_tux {
                tux = Some(logo.clone());
            }
            Some(logo)
        })
        .collect::<Vec<_>>();

    let tux = tux.unwrap();

    quote! { (#tux, [#(#logos),*]) }.into()
}
