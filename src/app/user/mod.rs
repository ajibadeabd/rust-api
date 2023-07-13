use rocket::Phase;


// // use rocket;
// use rocket::Rocket;
// pub struct Rocket<P: Phase>(pub(crate) P::State);
// pub mod user_route;
// pub fn mount(rocket: Rocket)->Rocket {
//     rocket.mount("/users", routes![user_route::test_handler2]);

// }
pub mod user_route;



// pub fn mount(rocket: Rocket<T>) {
//     rocket.mount("/users", routes![user_route::test_handler2])
// }
