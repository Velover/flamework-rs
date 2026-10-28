//! Layouts (`layoutOf`), the wire key and layout text (`wireKey`, `layoutText`), and use counts (`countUses`).

use rbxts_plugin::ts::{P, Type};
use tsrs_checker::TypeFlags;

use super::classify::datatype;
use super::order::literal_key;
use super::typeinfo::{TableKey, json_string};
use super::{GResult, Gen, Kind, Layout, Shape};

const BLOB_SIZE: u64 = 4;
const CFRAME_COMPONENTS: u64 = 12;

fn fixed(size: u64, blobs: bool) -> Layout {
    Layout { size: Some(size), min: size, blobs, zeros: false }
}

fn sum_layouts(layouts: &[Layout]) -> Layout {
    let mut size = Some(0);
    let mut min = 0;
    let mut blobs = false;
    let mut zeros = false;
    for layout in layouts {
        size = match (size, layout.size) {
            (Some(a), Some(b)) => Some(a + b),
            _ => None,
        };
        min += layout.min;
        blobs |= layout.blobs;
        zeros |= layout.zeros;
    }
    Layout { size, min, blobs, zeros }
}

/// `fieldKey(key)`.
fn field_key(key: &TableKey) -> String {
    match key {
        TableKey::Num(n) => crate::js::number_to_string(*n),
        TableKey::Str(s) => json_string(s),
    }
}

impl Gen<'_, '_, '_> {
    /// `layoutOf(shape)`.
    pub(crate) fn layout_of(&mut self, shape: &Shape) -> GResult<Layout> {
        let key = shape.key();
        if let Some(memo) = self.g.layouts.get(&key) {
            return Ok(*memo);
        }
        if self.g.visiting.contains(&key) {
            self.g.provisional += 1;
            return Ok(Layout { size: None, min: 0, blobs: false, zeros: false });
        }
        self.g.visiting.insert(key);
        let before = self.g.provisional;
        let layout = self.compute_layout(shape);
        self.g.visiting.remove(&key);
        let layout = layout?;
        if self.g.provisional == before || self.g.visiting.is_empty() {
            self.g.layouts.insert(key, layout);
        }
        if self.g.visiting.is_empty() {
            self.g.provisional = 0;
        }
        Ok(layout)
    }

    fn compute_layout(&mut self, shape: &Shape) -> GResult<Layout> {
        if let Shape::Type(ty) = shape {
            self.g.trail.push(*ty);
            let result = self.describe(shape).and_then(|kind| self.layout_of(&Shape::Kind(kind)));
            self.g.trail.pop();
            return result;
        }
        let kind = self.describe(shape)?;
        Ok(match &*kind {
            Kind::Number { width, .. } => fixed(u64::from(width.size()), false),
            Kind::Varint { .. } => Layout { size: None, min: 1, blobs: false, zeros: false },
            Kind::Boolean => fixed(1, false),
            Kind::String { length, .. } | Kind::Buffer { length, .. } => {
                Layout { size: None, min: u64::from(length.min()), blobs: false, zeros: false }
            }
            Kind::Constant(_) | Kind::Nothing => fixed(0, false),
            Kind::Literals(values) => fixed(if values.len() > 0xff { 2 } else { 1 }, false),
            Kind::Blob { .. } => fixed(BLOB_SIZE, true),
            Kind::Datatype(name) => {
                let (_, fields) = datatype(name).unwrap();
                fixed(fields.iter().map(|(w, _)| u64::from(w.size())).sum(), false)
            }
            Kind::CFrame => fixed(CFRAME_COMPONENTS * 4, false),
            Kind::Enum(_) => fixed(2, false),
            Kind::Optional(inner) => {
                let inner = self.layout_of(inner)?;
                Layout { size: None, min: 1, blobs: inner.blobs, zeros: inner.zeros }
            }
            Kind::Array(element) | Kind::Set(element) => {
                let element = self.layout_of(element)?;
                Layout { size: None, min: 1, blobs: element.blobs, zeros: element.size == Some(0) || element.zeros }
            }
            Kind::Map(key, value) => {
                let key = self.layout_of(key)?;
                let value = self.layout_of(value)?;
                Layout {
                    size: None,
                    min: 1,
                    blobs: key.blobs || value.blobs,
                    zeros: (key.size == Some(0) && value.size == Some(0)) || key.zeros || value.zeros,
                }
            }
            Kind::List { elements, rest, after } => {
                let mut layouts = Vec::new();
                for e in elements.iter().chain(after.iter().flatten()) {
                    layouts.push(self.layout_of(e)?);
                }
                let layout = sum_layouts(&layouts);
                match rest {
                    Some(rest) => {
                        let rest = self.layout_of(rest)?;
                        Layout {
                            size: None,
                            min: layout.min + 1,
                            blobs: layout.blobs || rest.blobs,
                            zeros: layout.zeros || rest.size == Some(0) || rest.zeros,
                        }
                    }
                    None => layout,
                }
            }
            Kind::Object { fields } => {
                let mut layouts = Vec::new();
                for f in fields {
                    layouts.push(self.layout_of(&f.shape)?);
                }
                sum_layouts(&layouts)
            }
            Kind::Union { alternatives, whole, .. } => {
                let mut members = Vec::new();
                for a in alternatives {
                    members.push(self.layout_of(&a.shape)?);
                }
                if whole.is_some() {
                    members.push(Layout { size: None, min: 1, blobs: false, zeros: false });
                }
                let first = members[0].size;
                let same = members.iter().all(|m| m.size == first);
                let size = if same { first.map(|s| 1 + s) } else { None };
                Layout {
                    size,
                    min: 1 + members.iter().map(|m| m.min).min().unwrap_or(0),
                    blobs: members.iter().any(|m| m.blobs),
                    zeros: members.iter().any(|m| m.zeros),
                }
            }
        })
    }

    /// `layoutText(shape)`.
    pub(crate) fn layout_text(&mut self, shape: &Shape) -> GResult<String> {
        Ok(self.wire_key(shape, &mut Vec::new(), true)?.0)
    }

    /// `listKey(list)`.
    pub(crate) fn list_key(&mut self, list: Shape) -> GResult<String> {
        Ok(self.wire_key(&list, &mut Vec::new(), false)?.0)
    }

    /// `wireKey(shape, stack, bytesOnly)`: the text and the outermost place on `stack` it refers to.
    pub(crate) fn wire_key(&mut self, shape: &Shape, stack: &mut Vec<Shape>, bytes_only: bool) -> GResult<(String, usize)> {
        if let Some(at) = stack.iter().position(|s| s.same(shape)) {
            return Ok((format!("^{}", stack.len() - at), at));
        }
        let cache = if bytes_only { &self.g.layout_keys } else { &self.g.wire_keys };
        if let Some(known) = cache.get(&shape.key()) {
            return Ok((known.clone(), usize::MAX));
        }
        let depth = stack.len();
        let kind = self.describe(shape)?;
        stack.push(shape.clone());
        let result = self.wire_key_text(&kind, stack, bytes_only);
        stack.pop();
        let (text, reaches) = result?;
        if reaches >= depth {
            let cache = if bytes_only { &mut self.g.layout_keys } else { &mut self.g.wire_keys };
            cache.insert(shape.key(), text.clone());
        }
        Ok((text, reaches))
    }

    fn wire_key_text(&mut self, kind: &Kind, stack: &mut Vec<Shape>, bytes_only: bool) -> GResult<(String, usize)> {
        let mut reaches = usize::MAX;
        let mut key = |gn: &mut Self, inner: &Shape, stack: &mut Vec<Shape>| -> GResult<String> {
            let (text, r) = gn.wire_key(inner, stack, bytes_only)?;
            reaches = reaches.min(r);
            Ok(text)
        };
        let checked = |implicit: bool| if implicit && !bytes_only { " checked" } else { "" };
        let text = match kind {
            Kind::Number { width, implicit } => format!("{}{}", width.name(), checked(*implicit)),
            Kind::Varint { implicit } => format!("varint{}", checked(*implicit)),
            Kind::String { length, implicit } => format!("string({}){}", length.name(), checked(*implicit)),
            Kind::Buffer { length, implicit } => format!("buffer({}){}", length.name(), checked(*implicit)),
            Kind::Constant(value) => format!("={}", literal_key(value)),
            Kind::Literals(values) => format!("literals({})", values.iter().map(literal_key).collect::<Vec<_>>().join(", ")),
            Kind::Blob { typeof_name } => match typeof_name {
                Some(name) if !bytes_only => format!("blob({name})"),
                _ => "blob".into(),
            },
            Kind::Datatype(name) => format!("datatype({name})"),
            Kind::Enum(name) => format!("enum({name})"),
            Kind::Optional(inner) => format!("optional({})", key(self, inner, stack)?),
            Kind::Array(element) => format!("array({})", key(self, element, stack)?),
            Kind::Set(element) => format!("set({})", key(self, element, stack)?),
            Kind::Map(k, v) => {
                let k = key(self, k, stack)?;
                let v = key(self, v, stack)?;
                format!("map({k}, {v})")
            }
            Kind::List { elements, rest, after } => {
                let mut parts = Vec::new();
                for e in elements {
                    parts.push(key(self, e, stack)?);
                }
                let rest = match rest {
                    Some(r) => format!("; ...{}", key(self, r, stack)?),
                    None => String::new(),
                };
                let after = match after {
                    Some(a) if !a.is_empty() => {
                        let mut keys = Vec::new();
                        for e in a {
                            keys.push(key(self, e, stack)?);
                        }
                        format!("; {}", keys.join(", "))
                    }
                    _ => String::new(),
                };
                format!("list({}{rest}{after})", parts.join(", "))
            }
            Kind::Object { fields } => {
                let mut parts = Vec::new();
                for f in fields {
                    parts.push(format!("{}: {}", field_key(&f.key), key(self, &f.shape, stack)?));
                }
                format!("object({})", parts.join(", "))
            }
            Kind::Union { alternatives, whole, .. } => {
                let mut parts = Vec::new();
                for a in alternatives {
                    parts.push(key(self, &a.shape, stack)?);
                }
                let whole = whole.map(|w| format!("; whole {w}")).unwrap_or_default();
                format!("union({}{whole})", parts.join(", "))
            }
            other => other.name().to_owned(),
        };
        Ok((text, reaches))
    }

    /// `countUses(root)`.
    pub(crate) fn count_uses(&mut self, root: &Shape) -> GResult<()> {
        *self.g.uses.entry(root.key()).or_insert(0) += 1;
        if let Shape::Type(ty) = root
            && self.can_hoist(*ty)?
        {
            if self.g.walked.contains(&root.key()) {
                return Ok(());
            }
            self.g.walked.insert(root.key());
        }
        let kind = self.describe(root)?;
        match &*kind {
            Kind::Optional(inner) => self.count_uses(inner)?,
            Kind::Array(element) | Kind::Set(element) => self.count_uses(element)?,
            Kind::Map(k, v) => {
                self.count_uses(k)?;
                self.count_uses(v)?;
            }
            Kind::List { elements, rest, after } => {
                for e in elements {
                    self.count_uses(e)?;
                }
                if let Some(r) = rest {
                    self.count_uses(r)?;
                }
                for e in after.iter().flatten() {
                    self.count_uses(e)?;
                }
            }
            Kind::Object { fields } => {
                for f in fields {
                    self.count_uses(&f.shape)?;
                }
            }
            Kind::Union { alternatives, .. } => {
                for a in alternatives {
                    self.count_uses(&a.shape)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// `canHoist(type)`.
    pub(crate) fn can_hoist(&mut self, ty: P<Type>) -> GResult<bool> {
        if !ty.flags().intersects(TypeFlags::Object | TypeFlags::UnionOrIntersection) {
            return Ok(false);
        }
        let shape = Shape::Type(ty);
        if self.layout_of(&shape)?.size.is_some() {
            return Ok(false);
        }
        Ok(self.describe(&shape)?.is_hoistable())
    }

    /// `carriesNothing(list)`.
    pub(crate) fn carries_nothing(&mut self, list: &Kind) -> GResult<bool> {
        let Kind::List { elements, rest, .. } = list else { return Ok(false) };
        if rest.is_some() {
            return Ok(false);
        }
        for e in elements {
            if !matches!(&*self.describe(e)?, Kind::Nothing) {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
