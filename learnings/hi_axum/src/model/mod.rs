use crate::{Result, Error} ;

use std::sync::Arc;

use serde::{Serialize, Deserialize};
use std::sync::Mutex;


#[derive(Debug, Clone, Serialize)]
pub struct Ticket{
    pub id: u64,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct TicketForCreate{
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController{
    // clone 只是拷贝最外侧的Arc
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController{
    pub async fn new() -> Result<ModelController>{
        Ok(Self{
            tickets_store: Arc::default(),
        })
    }
}

impl ModelController {
    
    pub async fn create_ticket(&self,ticket4c:TicketForCreate) -> Result<Ticket>{
        let mut store = self.tickets_store.lock().unwrap();
        let id  = store.len() as u64;

        let ticket = Ticket{
            id,
            title: ticket4c.title,
        };

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_tickets(&self,id: u64) -> Result<Ticket>{
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t|{
            t.take()
        });

        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id: id })
    }

}