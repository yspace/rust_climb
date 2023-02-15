pub trait Serializable<'a> {
    fn serialize(self: &Self) -> Cow<'a, str>;
}

struct Person<'a> {
    name: &'a str,
    age: i32,
}
struct Restaurant<'a> {
    name: &'a str,
    brunch: bool,
}

impl<'a> Serializable<'a> for Person<'a> {
    fn serialize(self: &Self) -> Cow<'a, str> {
        Cow::Owned(self.name.to_owned() + " " + &self.age.to_string())
    }
}

// 使用 type Classes
pub fn serialize_method<'a, T>(v: &T) -> Cow<'a, str>
where
    T: Serializable<'a>,
{
    T::serialize(v)
}

// 为内置类型实现我们的trait
impl<'a, T: Serializable<'a>> Serializable<'a> for Vec<T> {
    fn serialize(self: &Self) -> Cow<'a, str> {
        let result = self
            .iter()
            .map(|x| serialize_method(x))
            .collect::<Vec<Cow<'a, str>>>();
        let join_result = result.join(",");
        Cow::Owned(join_result)
    }
}
