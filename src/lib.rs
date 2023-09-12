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
			Ok(Resp { content: FastStr::from_static_str("success") })
		}else{
			Ok(Resp { content: FastStr::from_static_str("failed") })
		} 
		
	}
	async fn del_var(&self, req: volo_gen::myredis::Varible) -> ::core::result::Result<volo_gen::myredis::Resp, ::volo_thrift::AnyhowError>{
		//println!("Delete Var {}",req.key);
		let mut db = DATABASE.lock().unwrap();
		println!("Remove Val {}",req.key);
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
				Ok(Resp { content: url })
			},
			None => Ok(Resp { content: FastStr::from_static_str("pong") }), //return pong
		}
	}
}
