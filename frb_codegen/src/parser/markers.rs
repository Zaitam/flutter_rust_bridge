use syn::*;

/// Extract a path from marker `#[frb(mirror(path), ..)]`
pub fn extract_mirror_marker(attrs: &[Attribute]) -> Vec<Path> {
    attrs
        .iter()
        .filter(|attr| attr.path.is_ident("frb"))
        .find_map(|attr| match attr.parse_meta() {
            Ok(Meta::List(MetaList { nested, .. })) => nested.iter().find_map(|meta| match meta {
                NestedMeta::Meta(Meta::List(MetaList {
                    path,
                    nested: mirror,
                    ..
                })) if path.is_ident("mirror") && !mirror.is_empty() => Some(
                    mirror
                        .into_iter()
                        .filter_map(|label| {
                            if let NestedMeta::Meta(Meta::Path(path)) = label {
                                Some(path)
                            } else {
                                None
                            }
                        })
                        .cloned()
                        .collect::<Vec<_>>(),
                ),
                _ => None,
            }),
            _ => None,
        })
        .unwrap_or_default()
}

/// Checks if the `#[frb(non_final)]` attribute is present.
pub fn has_non_final(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .filter(|attr| attr.path.is_ident("frb"))
        .any(|attr| {
            match attr.parse_meta() {
            Ok(Meta::List(MetaList { nested, .. })) => nested.iter().any(|meta| {
                matches!(meta, NestedMeta::Meta(Meta::Path(path)) if path.is_ident("non_final"))
            }),
            _ => false,
        }
        })
}
