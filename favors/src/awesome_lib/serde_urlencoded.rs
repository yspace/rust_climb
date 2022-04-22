#[cfg(test)]
mod test_serialize {
    use serde_derive::Serialize;

    #[derive(Serialize)]
    struct NewType<T>(T);

    #[test]
    fn serialize_option_map_string() {
        let params = &[
            ("first", Some("hello")),
            ("middle", None),
            ("last", Some("world")),
        ];

        assert_eq!(
            serde_urlencoded::to_string(params),
            Ok("first=hello&last=world".to_owned())
        );
    }

    #[test]
    fn serialize_option_map_bool() {
        let params = &[("one", Some(true)), ("two", Some(false))];

        assert_eq!(
            serde_urlencoded::to_string(params),
            Ok("one=true&two=false".to_owned())
        );
    }
}

#[cfg(test)]
mod test_de {

    use serde_derive::Deserialize;

    #[derive(Deserialize, Debug, PartialEq)]
    struct NewType<T>(T);

    #[test]
    fn deserialize_newtype_i32() {
        let result = vec![("field".to_owned(), NewType(11))];

        assert_eq!(serde_urlencoded::from_str("field=11"), Ok(result));
    }

    #[test]
    fn deserialize_bytes() {
        let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

        assert_eq!(
            serde_urlencoded::from_bytes(b"first=23&last=42"),
            Ok(result)
        );
    }
}
