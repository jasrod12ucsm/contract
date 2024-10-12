use ntex::web::{scope, ServiceConfig};


pub fn content_type_scope(configure:&mut ServiceConfig){
    configure.service(
        scope("/")
    );
}