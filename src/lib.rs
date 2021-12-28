mod protogen{
    pub mod helloworld;
}

mod test{
    use crate::protogen::helloworld::{self, greeter_server::{Greeter, GreeterServer}};
    use crate::protogen::helloworld::{HelloReply, HelloRequest};
}