// clone是显式调用的 允许我们显式duplicate某物
// 游戏类比：类似一堆妖怪 其实都是从一个本体复制出来了 想起了女娲造人哈哈！
// 绘画类比：相当于 可以根据原物又画一个    
// `Clone` trait 有两个方法 
// - clone(&self) -> Self  此方法是`required` 需要被实现的方法
// - clone_from(&mut self, source: &Self) a.clone_from(&b) <=等价=> a = b.clone() ；
// 作画类比 就是有个物体 它可被mutable（可涂改的）相当于它开始是个虚画对象（或是橡皮擦掉一个已完成的对象细节） 你可以先画个轮廓 然后根据其他对象实画它
// 此方法是 `provided` 方法  就是自动为你提供的方法 
// 前提是你自己实现了clone 那么就自动拥有了此方法 类似于模板设计模式 你只要实现每个步骤方法 你便自动拥有整个模板功能

// youtube 博主说这个类似 模板设计模式来构造对象 估计他是说 `prototype`原型设计模式

// 一个最完备 traits 里面可能同时拥有 `required` `provided` 方法 。这东西在其他语言中 类比`interface` `abstract class`
// 一个类型 T 实现一个 trait ，你可能只需要derive 一下标记一下就行了 或者手动实现其required 方法  
// traits 用来充当 抽象类 这种用法就是给你提供额外实现 类似说你只要当我家孩子你就有这种能力了（对富豪就是钞能力）想起小鬼子，你只要当汉奸我就给你赋权 required方法集要求你有这个能力才能充当某个角色

pub fn run() {

    let mut player_location = Location{
        x: 10,
        y: 20,
    };

    let mut p2 = player_location.clone();

    // dbg!(p2) ; // 这样会move p2的！
    dbg!(&p2) ;
    p2.x = 30 ;

    player_location.clone_from(&p2) ;

    dbg!(&player_location) ;

    // usecase 2: 画画类比：先搞个默认轮廓占位 然后从其他对象照画下
    let mut p = Location::default();
      p.clone_from(&p2) ;
      dbg!(p) ;

}


#[derive(Debug,Default)]
struct Location{
    x: i32, 
    y: i32,
}

impl Clone for Location {
    fn clone(&self) -> Self {
        Self { x: self.x.clone(), y: self.y.clone() }
    }
}