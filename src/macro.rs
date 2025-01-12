/// ```ignore
/// (&json::object::Object, &str, identifer) -> Result(&JsonValue, String)
/// ```
/// # Example
/// ```ignore
/// let attribute_value = simple_get!(object, "attribute", function_name)
/// ```
macro_rules! simple_get {
    ($object: expr, $key: expr) => {
        $object.get($key).ok_or(Error(
            format!("wrong {} (not an attr). {:?}", $key, $object),
            None,
        ))
    };
}

pub(crate) use simple_get;

macro_rules! simple_get_as {
    ($object: expr, $key: expr, $as_type: ident) => {
        $object.get($key).and_then(|v| v.$as_type()).ok_or(Error(
            format!(
                "wrong {} (not an attr or invalid cast). {:?}",
                $key, $object
            ),
            None,
        ))
    };
}

pub(crate) use simple_get_as;
