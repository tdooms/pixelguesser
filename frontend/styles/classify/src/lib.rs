// struct FieldInfo {
//     class: Option<String>,
//     skip: bool,
//     name: Ident,
// }
//
// #[proc_macro_derive(Classify, attributes(class, skip))]
// pub fn derive(input: TokenStream) -> TokenStream {
//     let ast = parse_macro_input!(input as DeriveInput);
//
//     let fields = if let syn::Data::Struct(syn::DataStruct {
//         fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
//         ..
//     }) = ast.data
//     {
//         named
//     } else {
//         unimplemented!();
//     };
//
//     let extract = |field: &Field| -> FieldInfo {
//         let mut skip = false;
//         for attr in &f.attrs {
//             skip |= attr.path.segments.len() == 1 && attr.path.segments[0].ident == "skip";
//         }
//
//         let class = "is-hidden".to_owned();
//         let name = field.ident;
//
//         FieldInfo { skip, class, name }
//     };
//
//     let info: Vec<_> = fields.iter().map(extract).collect();
// }
