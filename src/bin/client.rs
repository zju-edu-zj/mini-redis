use lazy_static::lazy_static;
use pilota::FastStr;
use std::{
    io::Write,
    net::SocketAddr, 
};
use volo_gen::myredis::{Kv,Varible,PingReq};

lazy_static! {
    static ref CLIENT: volo_gen::myredis::RedisServeClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::myredis::RedisServeClientBuilder::new("mini-redis")
            // .layer_outer(LogLayer)
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
                if let Ok(resp) = CLIENT.get_var(req).await{
                    if resp.val==0x7fffffff{
                        println!("The key doesn't exist");
                    }else{
                        println!("{}",resp.val);
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
                            println!("{:?}", resp); //success of fail
                        }
                        Err(_resp)=>{
                            println!("Internet error");  //since we always return Ok
                        }
                    }
                }else{
                    println!("The value should be integer"); //format control
                }
                
            }
            "delete" => {
                let key = args[0];
                let req = Varible{key:FastStr::from(key.to_string())};
                match CLIENT.del_var(req).await{
                    Ok(resp) => {
                        println!("{:?}", resp); //success of fail
                    }
                    Err(_resp)=>{
                        println!("Internet error");  //since we always return Ok
                    }
                }
            }
            "ping" => {
                let msg = args.join(" "); //to a particular string
                let resp = if args.is_empty() {
                    CLIENT.ping(PingReq { url: None }).await  //nothing
                } else {
                    CLIENT.ping(PingReq { url: Some(FastStr::from(msg))}).await
                };
                println!("{:?}", resp); //a string
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