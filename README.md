### prospect-backend

#### api for frontend

|    api     | method |               frontend                |                  backend                  |
|:----------:|:------:|:-------------------------------------:|:-----------------------------------------:|
|     /      |   /    |                   /                   |                     /                     |
|  /signin   |  post  |    json: [SignUpInfo](#SignUpInfo)    |    json: [SignUpResult](#SignUpResult)    |
|   /login   |  post  |     json: [LogInInfo](#LogInInfo)     |     json: [LogInResult](#LogInResult)     |
| /subscribe |  post  | json: [SubscribeInfo](#SubscribeInfo) | json: [SubscribeResult](#SubscribeResult) |
|  /search   |  post  |    json: [SearchInfo](#SearchInfo)    |    json: [SearchResult](#SearchResult)    |

##### SignUpInfo
```rust
struct SignUpInfo {
  username: String,
  password: String,
}
```

##### SignUpResult
```rust
struct SignUpResult {
  success: bool,
  message: String,
}
```

##### LogInInfo
```rust
struct LogInInfo {
  username: String,
  password: String,
  // verify code: String,
}
```

##### LogInResult
```rust
struct LogInResult {
  success: bool,
  message: String,
  access_token: String,
}
```

##### SubscribeInfo
```rust
struct SubscribeInfo {
  access_token: String,
  cmd: u8  // 0: unsubscribe, 1: subscribe
  // TODO: tag_to_subscribe: ??
}
```

##### SubscribeResult
```rust
struct SubscribeResult {
  success: bool,
  message: String,
}
```

##### SearchInfo
```rust
struct SearchInfo {
  access_token: String,
  keyword: String,
}
```

##### SearchResult
```rust
struct SearchResult {
  success: bool,
  message: String,
  // TODO: result: ??
}
```
