use std::str::FromStr;

pub fn parse_into_vector<T, U>(buf: &str, mapping: U) -> Vec<T>
where
    T: FromStr,
    U: Fn(&str) -> Result<T, T::Err>,
{
    buf.trim().split_whitespace().flat_map(mapping).collect()
}
