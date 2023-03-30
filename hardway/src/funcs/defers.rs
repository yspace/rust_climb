use color_eyre::owo_colors::OwoColorize;

type FnTask = Box<dyn Fn()->()>;

#[derive(Default)]
struct DeferTask{
    tasks: Vec<FnTask>,
}

impl DeferTask {
    fn add_task(&mut self, task: FnTask){
        self.tasks.push(task);
    }
    fn add_task2(&mut self, task: impl Fn()->()+'static){
        self.tasks.push(Box::new(task));
    }
}

// 利用析构函数实现 逆序调用
impl Drop for DeferTask {
    fn drop(&mut self) {
        
        // for task in self.tasks. iter().reversed() {
        //     task();
        // }
       while let Some(task) = self.tasks.pop(){
        task();
       }
    }
}

#[test]
fn it_woks(){
    let mut t = DeferTask::default();

    t.add_task(Box::new(||{
        println!("1");
    }));
    t.add_task(Box::new(||{
        println!("2");
    }));
    t.add_task(Box::new(||{
        println!("3");
    }));

    t.add_task2(||{
        println!("4");
    });

    
}