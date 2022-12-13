use std::{any::{Any, TypeId}, collections::HashMap};


#[derive(Debug,Default)]
pub struct Entities{
    components: HashMap<TypeId,Vec<Option<Box<dyn Any>>>> ,
    entity_count: usize,
 }


 impl Entities{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_entity<T: 'static + Sized>(&mut self) -> &mut Self{
        self.entity_count += 1;
        self.add_empty_cell_to_all_components() ;
        let type_id = TypeId::of::<T>();
        if    self.components.get_mut(&type_id).is_none() {
            self.add_component(type_id) ;
        }
        
        self
    }

    pub fn with_component(&mut self,component: impl Any) -> Result<&mut Self> {
        let type_id = component.type_id();
        if let Some(components) = self.components.get_mut(&type_id) {
            if let Some(last_component) = components.last_mut() {
                *last_component = Some(Box::new(component)) ;
            }
        }else{
            bail!("could not find components
            to insert into");
        }

        Ok(self)
    }


    pub fn add_empty_cell_to_all_components(&mut self){
        for component in self.components.values_mut() {
            component.push(None)
        }
    }

    pub fn add_component(&mut self, type_id: TypeId){
        let mut components = vec![] ;
        for i in 0..self.entity_count {
            components.push(None) ;
        }

        self.components.insert(type_id, components) ;
    }
 }

 #[cfg(test)]
 mod test {
    use anyhow::Ok;

    use super::*;

    struct Size(f32) ;
    #[test]
    fn test_adding_entity() -> Result<()> {


        let mut entities = Entities::new();
        entities.register_entity::<Size>()
        .with_component(Size(10.0))?;

        Ok(())
    }
 }