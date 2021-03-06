// Copyright (c) Facebook, Inc. and its affiliates.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[derive(Debug, Clone)]
pub struct BelowViewAttr {
    pub depth: Option<proc_macro2::TokenStream>,
    pub unit: Option<String>,
    pub prefix: Option<proc_macro2::TokenStream>,
    pub width: Option<usize>,
    pub title_width: Option<usize>,
    pub none_mark: String,
    pub decorator: Option<String>,
    pub precision: Option<usize>,
    pub highlight_if: Option<String>,
}

#[derive(Default, Debug, Clone)]
pub struct BelowFieldAttr {
    pub title: Option<String>,
    pub link: Vec<String>,
    pub tag: Option<String>,
    pub sort_tag: Option<String>,
    pub class: Option<String>,
    pub no_show: bool,
    pub cmp: bool,
    pub gen: bool,
    pub class_detail: bool,
    pub dfill_struct: Option<String>,
}

#[derive(Default, Debug)]
pub struct BelowAttr {
    pub view: Option<BelowViewAttr>,
    pub field: Option<BelowFieldAttr>,
}

impl std::default::Default for BelowViewAttr {
    fn default() -> Self {
        Self {
            depth: None,
            unit: None,
            prefix: None,
            width: None,
            title_width: None,
            none_mark: "?".into(),
            decorator: None,
            precision: None,
            highlight_if: None,
        }
    }
}

/// Parse attribute into attribute data structure.
/// If there's a field, there will be a view, but not vice versa.
pub fn parse_attribute(attrs: &[syn::Attribute], field_name: &syn::Ident) -> BelowAttr {
    let mut bttr: BelowAttr = std::default::Default::default();
    let mut bfttr: BelowFieldAttr = std::default::Default::default();
    let mut bvttr: BelowViewAttr = std::default::Default::default();
    let mut field_flag = false;
    let mut view_flag = false;

    if attrs.is_empty() {
        return bttr;
    }

    attrs
        .iter()
        .filter(|a| a.path.segments[0].ident == "blink")
        .for_each(|a| match a.parse_meta() {
            Ok(m) => match m {
                syn::Meta::List(m_list) => {
                    if m_list.nested.len() != 1 {
                        unimplemented!(
                            "{}: Currently blink attribute only support 1 link",
                            field_name
                        );
                    }

                    let n_meta = m_list.nested.first().unwrap();
                    match n_meta {
                        syn::NestedMeta::Lit(syn::Lit::Str(ls)) => {
                            field_flag = true;
                            bfttr.link.push(ls.value());
                        }
                        _ => unimplemented!("{}: blink value has to be a string", field_name),
                    }
                }
                _ => unimplemented!("{}: Fail to parse bttr meta list", field_name),
            },
            _ => unimplemented!("{}: Fail to parse meta in bttr", field_name),
        });

    attrs
        .iter()
        .filter(|a| a.path.segments[0].ident == "bttr")
        .map(|a| match a.parse_meta() {
            Ok(m) => match m {
                syn::Meta::List(m_list) => m_list.nested.into_iter(),
                _ => unimplemented!("{}: Fail to parse bttr meta list", field_name),
            },
            _ => unimplemented!("{}: Fail to parse meta in bttr", field_name),
        })
        .flatten()
        .map(|n_meta| match n_meta {
            syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) => nv,
            _ => unreachable!(),
        })
        .for_each(|nv| {
            match format!("{}", &nv.path.segments[0].ident).as_str() {
                "title" => {
                    field_flag = true;
                    view_flag = true;
                    bfttr.title = match &nv.lit {
                        syn::Lit::Str(ls) => Some(ls.value()),
                        _ => unimplemented!("{}: title has to be a string", field_name),
                    }
                }
                "sort_tag" => {
                    field_flag = true;
                    bfttr.sort_tag = match &nv.lit {
                        syn::Lit::Str(ls) => Some(ls.value()),
                        _ => unimplemented!("{}: sort_tag has to be a string", field_name),
                    };
                    bfttr.cmp = true;
                }
                "tag" => {
                    field_flag = true;
                    bfttr.tag = match &nv.lit {
                        syn::Lit::Str(ls) => Some(ls.value()),
                        _ => unimplemented!("{}: tag has to be a string", field_name),
                    };
                }
                "class" => {
                    bfttr.class = match &nv.lit {
                        syn::Lit::Str(ls) => Some(ls.value()),
                        _ => unimplemented!("{}: class has to be a string", field_name),
                    }
                }
                "dfill_struct" => {
                    bfttr.dfill_struct = match &nv.lit {
                        syn::Lit::Str(ls) => Some(ls.value()),
                        _ => unimplemented!("{}: dfill_struct has to be a string", field_name),
                    }
                }
                "unit" => {
                    view_flag = true;
                    bvttr.unit = match &nv.lit {
                        syn::Lit::Str(ls) => Some(ls.value()),
                        _ => unimplemented!("{}: unit has to be a string", field_name),
                    }
                }
                "decorator" => {
                    view_flag = true;
                    bvttr.decorator = match &nv.lit {
                        syn::Lit::Str(ls) => Some(ls.value()),
                        _ => unimplemented!("{}: decorator has to be a string", field_name),
                    }
                }
                "depth" => {
                    view_flag = true;
                    bvttr.depth = match &nv.lit {
                        syn::Lit::Str(ls) => Some(ls.value().parse().unwrap()),
                        _ => unimplemented!("{}: depth has to be a string", field_name),
                    }
                }
                "prefix" => {
                    view_flag = true;
                    bvttr.prefix = match &nv.lit {
                        syn::Lit::Str(ls) => Some(ls.value().parse().unwrap()),
                        _ => unimplemented!("{}: prefix has to be a string", field_name),
                    }
                }
                "width" => {
                    view_flag = true;
                    bvttr.width = match &nv.lit {
                        syn::Lit::Int(li) => {
                            Some(li.base10_parse::<usize>().expect("Fail to parse width"))
                        }
                        _ => unimplemented!("{}: width has to be a integer", field_name),
                    }
                }
                "title_width" => {
                    view_flag = true;
                    bvttr.title_width = match &nv.lit {
                        syn::Lit::Int(li) => Some(
                            li.base10_parse::<usize>()
                                .expect("Fail to parse title_width"),
                        ),
                        _ => unimplemented!("{}: width has to be a integer", field_name),
                    }
                }
                "precision" => {
                    view_flag = true;
                    bvttr.precision = match &nv.lit {
                        syn::Lit::Int(li) => {
                            Some(li.base10_parse::<usize>().expect("Fail to parse precision"))
                        }
                        _ => unimplemented!("{}: precision has to be a integer", field_name),
                    }
                }
                "none_mark" => {
                    view_flag = true;
                    bvttr.none_mark = match &nv.lit {
                        syn::Lit::Str(ls) => ls.value(),
                        _ => unimplemented!("{}: none_mark has to be a string", field_name),
                    }
                }
                "cmp" => {
                    field_flag = true;
                    bfttr.cmp = match &nv.lit {
                        syn::Lit::Bool(lb) => lb.value,
                        _ => unimplemented!("{}: cmp has to be a boolean", field_name),
                    }
                }
                "highlight_if" => {
                    view_flag = true;
                    bvttr.highlight_if = match &nv.lit {
                        syn::Lit::Str(ls) => Some(ls.value()),
                        _ => unimplemented!("{}: highlight_if has to be a string", field_name),
                    }
                }
                "gen" => {
                    field_flag = true;
                    bfttr.gen = match &nv.lit {
                        syn::Lit::Bool(lb) => lb.value,
                        _ => unimplemented!("{}: gen has to be a boolean", field_name),
                    }
                }
                "class_detail" => {
                    field_flag = true;
                    bfttr.class_detail = match &nv.lit {
                        syn::Lit::Bool(lb) => lb.value,
                        _ => unimplemented!("{}: class_detail has to be a boolean", field_name),
                    }
                }
                _ => unimplemented!("{}: Unknown field", field_name),
            };
        });

    if field_flag {
        bttr.field = Some(bfttr);
    }

    if view_flag {
        bttr.view = Some(bvttr);
    }

    bttr
}
