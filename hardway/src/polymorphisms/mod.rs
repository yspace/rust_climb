// @see https://www.youtube.com/watch?v=CHRNj5oubwc&ab_channel=CodetotheMoon
pub trait LandCapable{
    fn drive(&self) {
        println!("i can drive");
    }
}
pub trait WaterCapable{
    fn float(&self){
        println!("i can float");
    }
}

pub trait Amphibious: LandCapable + WaterCapable {}

// want anything that implements both `LandCapable` and `WaterCapable` to automatically implement `Amphibious` as well

impl<T: LandCapable + WaterCapable> Amphibious for T {}
// Which can be read as "implement `Amphibious` for any type T that implements `LandCapable` and `WaterCapable`".
// There is also alias traits in unstable, that basically do more or less the same thing but with less boilerplate:

// trait Amphibious = LandCapable + WaterCapable;

struct Sedan;
impl LandCapable for Sedan {
    
}
struct SUV;
impl LandCapable for SUV{}

// 接受动态trait作为参数的方法 意味着完成这件事的参与者 只需要满足这个trait就行 不管你具体是啥类型
fn road_trip(vehicle: &dyn LandCapable){
    vehicle.drive();
}
fn road_trip2(vehicle: &impl LandCapable){
    vehicle.drive();
}

// 思考 程序 = 数据结构 + 算法
// 算法 一般是系统中动的部分 
// 结构 是系统中静的部分 二者如同☯️太极  对于trait 都可以作为这二者的依赖 或作为参数 或者作为成员变量

// 对于地球这个系统而言 纵观古今 无数国家在其中兴亡 地球承载的就是这些国家 具体是谁并不重要 因为终将灭亡替换 
// 国家就可以是一个Trait 可被抽象对待 国家的职能就可以描述为其中的方法

struct Hovercraft;
// impl Amphibious for Hovercraft {
    
// }
impl LandCapable for Hovercraft {
    
}
impl WaterCapable for Hovercraft {
    // 这个是覆盖默认实现了
    fn float(&self){
        println!("i can float; and i am Hovercraft");
    }
}

fn traverse_frozen_lake(vehicle: &dyn Amphibious){
    vehicle.drive();
    vehicle.float() ;
}

#[test]
fn test_it(){
    println!("hi polymorphisems");
    let vehicle = Hovercraft;

    traverse_frozen_lake(&vehicle);
}