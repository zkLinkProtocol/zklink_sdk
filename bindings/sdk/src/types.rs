use std::sync::Arc;

pub fn to_json<T>(_t: Arc<T>) -> String{
    "hello".into()
}
