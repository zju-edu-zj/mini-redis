namespace rs myredis;

struct Varible{
    1: required string key,
}

struct KV{
    1: required string key,
    2: required i64 val,
}

struct Value{
    1: required i64 val,
}

struct Resp{
    1: required string content,
}

struct pingReq{
    1: optional string url,
}
service RedisServe{
    Value GetVar(1: Varible req),
    Resp SetVar(1: KV req),
    Resp DelVar(1: Varible req),
    Resp ping(1: pingReq req),
}