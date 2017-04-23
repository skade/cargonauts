use std::io;
use std::cell::RefCell;
use std::rc::Rc;

use Error;

use anymap::AnyMap;
use futures::{Future, future, Stream, stream};
use core::reactor::Handle;
use tokio::NewService;
use c3po::{ConnFuture, Conn, Pool, Config};

use connections::{Client, Configure};

#[derive(Clone)]
pub struct Environment {
    pools: Rc<AnyMap>,
}

impl Environment {
    pub fn conn<C: NewService>(&self) -> ConnFuture<Conn<C>, Error> {
        if let Some(pool) = self.pools.get::<Pool<C>>() {
            pool.connection()
        } else {
            future::Either::A(future::err(Error))
        }
    }

    pub fn conn_for<C: NewService>(&self, name: &'static str) -> ConnFuture<Conn<C>, Error> {
        if let Some(pools) = self.pools.get::<Vec<(&'static str, Pool<C>)>>() {
            if let Some(&(_, ref pool)) = pools.iter().find(|&&(s, _)| s == name) {
                pool.connection()
            } else {
                future::Either::A(future::err(Error))
            }
        } else {
            future::Either::A(future::err(Error))
        }
    }

    pub fn client<C: Client>(&self) -> Box<Future<Item = C, Error = Error>> {
        if let Some(pool) = self.pools.get::<Pool<C::Connection>>() {
            Box::new(pool.connection().map(|c| C::connect(c)))
        } else if let Some(pools) = self.pools.get::<Vec<(&'static str, Pool<C::Connection>)>>() {
            if let Some(&(_, ref pool)) = pools.iter().find(|&&(s, _)| s == C::CONNECTION_NAME) {
                Box::new(pool.connection().map(|c| C::connect(c)))
            } else {
                Box::new(future::err(Error))
            }
        } else {
            Box::new(future::err(Error))
        }
    }
}

#[derive(Clone)]
pub struct EnvBuilder {
    pools: Rc<RefCell<AnyMap>>,
}

impl EnvBuilder {
    pub fn new() -> EnvBuilder {
        EnvBuilder {
            pools: Rc::new(RefCell::new(AnyMap::new())),
        }
    }

    pub fn new_pool<C>(self, handle: Handle, cfg: Config, client_cfg: C::Config)
        -> Box<Future<Item = (), Error = io::Error>>
    where C: NewService + Configure<Handle> + 'static
    {
        let client = C::new(client_cfg, handle.clone());
        Box::new(Pool::new(client, handle, cfg).map(move |pool| {
            self.pools.borrow_mut().insert(pool);
        }))
    }

    // For multiple connections of the same protocol
    pub fn new_pool_vec<C>(self, handle: Handle, cfgs: Vec<(&'static str, Config, C::Config)>)
        -> Box<Future<Item = (), Error = io::Error>>
    where C: NewService + Configure<Handle> + 'static
    {
        Box::new(stream::futures_unordered(cfgs.into_iter().map(|(name, cfg, client_cfg)| {
            let client = C::new(client_cfg, handle.clone());
            Pool::new(client, handle.clone(), cfg).map(move |pool| (name, pool))
        })).collect().map(move |pools| {
            self.pools.borrow_mut().insert(pools);
        }))
    }

    pub fn build(self) -> Environment {
        Environment {
            pools: Rc::new(Rc::try_unwrap(self.pools).unwrap().into_inner()),
        }
    }
}