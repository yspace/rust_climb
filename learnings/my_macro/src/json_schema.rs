
#[derive(Debug,Deserialize,Serialize,Default)]
struct Schema{
    title: Option<String>,
    #[serd(rename = "type")]
    typ: String,

    properties: Option<HashMap<String,Schema>>,
}
/// output Struct
pub struct St{
    pub name: String,
    pub fields: Vec<Fd>,
}

pub struct Fd{
    pub name: String,
    pub typ: String,
}

impl St{
    pub fn new(name: impl Into<String>, fields: Vec<Fd>) -> Self{
        Self{
            name: name.into(),
            fields,
        }
    }   
}

impl Fd{
    pub fn new(name: impl Into<String>, typ: Into<String>) -> Self{
        Self{
            name: name.into(),
            typ: typ.into(),
        }
    }
}

// impl From<Schema> for St{
//     fn from(schema: Schema) -> Self{
//         let mut fields = Vec::new();
//         if let Some(properties) = schema.properties{
//             for (name,schema) in properties{
//                 let fd = Fd::new(name,schema.typ);
//                 fields.push(fd);
//             }
//         }
//         St::new(schema.title.unwrap_or_else(|| "".to_string()),fields)
//     }
// }

// rust 不支持嵌套定义struct 所以 一个Schema 可能生出多个struct结构
impl Schema {
    pub fn into_string(&self) -> Vec<St>{
        let mut structs = Vec::new();

        structs 
    }
}