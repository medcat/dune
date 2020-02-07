use super::value::Value;

impl Into<MultiValue> for Value {
    fn into(self) -> MultiValue {
        MultiValue::Single(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MultiValue {
    Single(Value),
    Multi(Vec<Value>),
}

impl MultiValue {
    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        match self {
            MultiValue::Single(v) => MultiValueIter::Single(Some(&v)),
            MultiValue::Multi(v) => MultiValueIter::Multi((&v[..]).into_iter()),
        }
    }
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub enum MultiValueIntoIter {
    Single(Option<Value>),
    Multi(<Vec<Value> as IntoIterator>::IntoIter),
}

impl IntoIterator for MultiValue {
    type Item = Value;
    type IntoIter = MultiValueIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            MultiValue::Single(v) => MultiValueIntoIter::Single(Some(v)),
            MultiValue::Multi(v) => MultiValueIntoIter::Multi(v.into_iter()),
        }
    }
}

impl Iterator for MultiValueIntoIter {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            MultiValueIntoIter::Single(v) => v.take(),
            MultiValueIntoIter::Multi(v) => v.next(),
        }
    }
}

#[derive(Debug, Clone)]
enum MultiValueIter<'m> {
    Single(Option<&'m Value>),
    Multi(<&'m [Value] as IntoIterator>::IntoIter),
}

impl<'m> Iterator for MultiValueIter<'m> {
    type Item = &'m Value;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            MultiValueIter::Single(v) => v.take(),
            MultiValueIter::Multi(v) => v.next(),
        }
    }
}
