pub mod configuration ;
pub mod routes ;
pub mod startup ;


// async fn greet(req: HttpRequest) -> impl Responder {
//     // You can only use impl Trait if your function returns a single type;
//     //  if you want to return multiple, you need dynamic dispatch
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }


