use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for substr in s.split('&') {
            let mut key = substr;
            let mut val = "";

            if let Some(i) = substr.find('=') {
                key = &substr[..i];
                val = &substr[i+1..];

                data.entry(key).and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                      *existing = Value::Multiple(vec![prev_val, val]);
                    },
                    Value::Multiple(vec) => vec.push(val),
                }).or_insert(Value::Single(val));
            }
        }

        QueryString{data}
    }
}