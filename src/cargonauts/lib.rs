#![feature(associated_consts)]

pub extern crate futures;
extern crate c3po;
extern crate mainsail;
extern crate rigging;
pub extern crate serde;
pub extern crate serde_json as json;

#[allow(unused_imports)]
#[macro_use] extern crate compass_rose;
#[macro_use] extern crate proc_macro_hack;

#[doc(hidden)]
pub use compass_rose::*;

proc_macro_item_decl! {
    /// The routes DSL
    routes! => routes_impl
}

#[macro_use]
pub mod api {
    pub use rigging::{Resource, Error};
    pub use rigging::environment::Environment;
    pub use mainsail::methods::{Get, Index};

    #[macro_use]
    pub mod relations {
        pub use rigging::Relationship;
        pub use mainsail::methods::{GetOne, GetMany};

        #[macro_export]
        macro_rules! relation {
            ($rel:ident => $resource:ident) => {
                pub struct $rel;

                impl $crate::api::relations::Relationship for $rel {
                    type Related = $resource;
                }
            }
        }
    }
}

#[doc(hidden)]
pub mod routing {
    pub use rigging::{ResourceEndpoint, RelationEndpoint, RelationshipLink};
    pub use rigging::endpoint::endpoint;
    pub use rigging::routes::{Kind, RoutingTable, RouteKey, Handler, not_found};
    pub use rigging::environment::EnvBuilder;
}

pub mod server {
    pub use rigging::http::{Request, Response, Error, Service, NewService, serve, Handle};

    pub mod pool {
        pub use rigging::connections::Configure;
        pub use c3po::{Pool, Config};
    }
}

pub mod clients {
    pub use rigging::connections::{Client, Configure};
    pub use c3po::Conn;
}

pub mod method {
    pub use rigging::Method;
    pub use rigging::request::{Request, ResourceRequest, CollectionRequest};
    pub use rigging::routes::Route;
    pub use mainsail::methods::{GetRequest, IndexRequest};
}

pub mod format {

    pub use mainsail::formats::Debug;

    pub mod presenter {
        pub use rigging::format::{Present, PresentResource, PresentCollection, Template};
    }

    pub mod receiver {
        pub use rigging::format::Receive;
    }
}