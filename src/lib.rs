#![feature(impl_trait_in_assoc_type)]

use std::{
    collections::HashMap,
    //future::Future,
    sync::{Arc, Mutex},
};
use pilota::FastStr;

use lazy_static::lazy_static;

//use tracing_subscriber::fmt::format;

//use volo_thrift::ResponseError;
use anyhow::{Ok, anyhow};

use volo_gen::myredis::{Value,Resp}; //,Kv,Varible,PingReq};

lazy_static! {
    static ref DATABASE: Arc<Mutex<HashMap<FastStr, i64>>> = Arc::new(Mutex::new(HashMap::new()));
}
//static mut PROHIBWORD: String = String::from("123"); //you can set it to other value through 'SET' command too
pub struct S;

#[volo::async_trait]
impl volo_gen::myredis::RedisServe for S {
	async fn get_var(&self, req: volo_gen::myredis::Varible) -> ::core::result::Result<volo_gen::myredis::Value, ::volo_thrift::AnyhowError>{
		println!("Get Var {}",req.key);
		let db = DATABASE.lock().unwrap();
		match db.get(&req.key){
			Some(value) => Ok(Value { val: *value }),
			None => Ok(Value{val:0x7fffffff}), //infinity to represent not existing
		}
	}
	async fn set_var(&self, req: volo_gen::myredis::Kv) -> ::core::result::Result<volo_gen::myredis::Resp, ::volo_thrift::AnyhowError>{
		println!("Set Var {} to {}",req.key.to_string(),req.val);
		let mut db = DATABASE.lock().unwrap();
		if let Some(_) = db.insert(req.key, req.val){  //will overwrite it
			Ok(Resp { content: FastStr::from_static_str("updated") })
		}else{
			Ok(Resp { content: FastStr::from_static_str("inserted") })
		} 
		
	}
	async fn del_var(&self, req: volo_gen::myredis::Varible) -> ::core::result::Result<volo_gen::myredis::Resp, ::volo_thrift::AnyhowError>{
		//println!("Delete Var {}",req.key);
		let mut db = DATABASE.lock().unwrap();
		println!("Remove Var {}",req.key);
		if let Some(_val) = db.get(&req.key){
			db.remove(&req.key);
			Ok(Resp { content: FastStr::from_static_str("success") }) //always ok
		}else{
			Ok(Resp { content: FastStr::from_static_str("Not existed") })
		}
	}
	async fn ping(&self, req: volo_gen::myredis::PingReq) -> ::core::result::Result<volo_gen::myredis::Resp, ::volo_thrift::AnyhowError>{
		match req.url{
			Some(url) => {
				Ok(Resp { content: url })  //return directly
			},
			None => Ok(Resp { content: FastStr::from_static_str("pong") }), //return pong
		}
	}
}

#[derive(Clone)]
pub struct FilterService<S>(S);

#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for FilterService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug,
    Cx: Send + 'static,
	anyhow::Error: Into<S::Error>, //implemented for most errors
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        //since req has different types and all implement fmt, we can just convert it to string and filter some chars
		let req_str = format!("{:?}",req);
		{
			let db = DATABASE.lock().unwrap();
			if let Some(val) = db.get(&FastStr::from("PROHIB")){ //has prohibited word
				if req_str.contains(&val.to_string()){  //filter away
					return Err(anyhow!("prohibited").into()); //return directly
				}
			}
		}
        let resp = self.0.call(cx, req).await;
        tracing::debug!("Sent response {:?}", &resp);
        //tracing::info!("Request took {}ms", now.elapsed().as_millis());
        resp
    }
}

pub struct FilterLayer;

impl<S> volo::Layer<S> for FilterLayer {
    type Service = FilterService<S>;

    fn layer(self, inner: S) -> Self::Service {
        FilterService(inner)
    }
}
