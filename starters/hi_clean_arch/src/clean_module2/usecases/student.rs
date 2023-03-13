use crate::clean_module2::entities::repos::IUserRepo;


// 功能等价 控制器｜Service
// A good rule to start with is a class per use case
// 上面这条规则有点像早期java中的structs的Action ；需要贯彻每用例一个类的准则么？^_^
pub  struct StudentInteractor{
    student_repo: Box< dyn IUserRepo>
    // 注意 可能依赖多个repo哦 ，还可能依赖外部服务组件

}