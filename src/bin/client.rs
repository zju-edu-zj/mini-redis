use lazy_static::lazy_static;
use pilota::FastStr;
use std::{
    io::Write,
    net::SocketAddr, 
};
use volo_gen::myredis::{Kv,Varible,PingReq};
use mini_redis::FilterLayer;

lazy_static! {
    static ref CLIENT: volo_gen::myredis::RedisServeClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::myredis::RedisServeClientBuilder::new("mini-redis")
            .layer_outer(FilterLayer)
            .address(addr)
            .build()
    };
}

#[volo::main]
async fn main(){
    loop {
        print!("> ");
        std::io::stdout().flush().expect("failed to flush stdout");

        let mut input = String::new();

        std::io::stdin().read_line(&mut input).expect("failed to read from stdin");

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let mut args = input.split_whitespace(); //iterator
        let cmd = args.next().unwrap();
        let args = args.collect::<Vec<_>>();

        match cmd {
            "get" => {
                let key = args[0];
                let req = Varible{key:FastStr::from(key.to_string())}; //clone once
                //println!("{:?}", req);
                match CLIENT.get_var(req).await{
                    Ok(resp) =>{
                        if resp.val==0x7fffffff{
                            println!("The key doesn't exist");
                        }else{
                            println!("{}",resp.val);
                        }
                    }
                    Err(resp) =>{
                        println!("{:?}",resp);  //perhaps prohibited
                    }
                }
            }
            "set" => {
                let key = args[0];
                let value = args[1];
                if let Ok(inte) = value.parse::<i64>(){
                    let req = Kv{key:FastStr::from(key.to_string()),val:inte};
                
                    match CLIENT.set_var(req).await{
                        Ok(resp) => {
                            println!("{:?}", resp.content); //success or fail
                        }
                        Err(resp)=>{
                            println!("{:?}",resp);  //perhaps prohibited
                        }
                    }
                }else{
                    println!("The value should be integer"); //format control
                }
                
            }
            "del" => {
                let key = args[0];
                let req = Varible{key:FastStr::from(key.to_string())};
                match CLIENT.del_var(req).await{
                    Ok(resp) => {
                        println!("{:?}", resp.content); //success of fail
                    }
                    Err(resp)=>{
                        println!("{:?}",resp);  //perhaps prohibited
                    }
                }
            }
            "ping" => {
                let msg = args.join(" "); //to a particular string
                let result = if args.is_empty() {
                    CLIENT.ping(PingReq { url: None }).await  //nothing
                } else {
                    CLIENT.ping(PingReq { url: Some(FastStr::from(msg))}).await
                };
                match result{
                    Ok(resp)=>{
                        println!("{:?}",resp.content); //a string
                    }
                    Err(resp)=>{
                        println!("{:?}", resp); 
                    }
                }
                
            }
            "exit" => {
                break;
            }
            _ => {
                println!("unknown command: {}", cmd);
            }
        }
    }
}