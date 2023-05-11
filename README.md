# Actix Notes:

* HttpServiceFactory (trait)
- This trait is use in `HttpServer::new(|| { App::new().service(...) })`
- This trait can contrains multiple route and we can also add scope (parent path) as well
- from the scope we can also add service.

- We can add nested services (like parent scope and service -> sub scope and sub services)

- There is main Struct `HttpServer` in which we can add multiple services and sub services as well (HttpServer struct use builder pattern).
---

## web::Data<T>
- This is structure is to share state to the server
- This struct is work as `ARc` and we can make `Mutex<T>` to make Shared mutable State.
- We can add mutiple data which can share to all routes just need to add args in function.
- IDK how it's mange args order we can take any order or any amount of arg
---

## Configure <i>App::configure(self, f: FnOnce(&mut ServiceConfig))</i>
- We can create the function in which we can define service, routes, state.
- We can also seperate the State by Scope and Organize our code.

* Guard <i>App::guard(self, g: Guard)</i>
- By this function we can put some restriction on request comming from the client.
- like checking host, headers etc.

### Resource
- By this functionality we can set state for specific path. or add some Configuration on specific path. 
<!--  TODO: I have tried but geeting issue in state sharing for perticular path -->

