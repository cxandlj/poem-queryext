
 Implement poem FromRequest to deserialize struct from query string.

 #### Example
 ```no_run
 use poem_queryext::QueryExt;
 use poem_openapi::{payload::PlainText, OpenApi};
 use serde::Deserialize;

 struct Api;

 #[OpenApi]
 impl Api {
   //test url: /test?name=cx  
   //test url: /test?name=cx&age=18&hobby[0]=music&hobby[1]=game  
   #[oai(path = "/test", method = "get")]
   async fn test(&self, query: QueryExt<'_, QueryObj>) -> PlainText<String> {
        let obj = query.0;
        PlainText(format!(
            "name:{},age:{},hobby:{}",
            obj.name,
            obj.age.unwrap_or_default(),
            obj.hobby.unwrap_or_default().join(",")
        ))
    }
 }

 #[derive(Deserialize)]
 #[serde(rename_all = "camelCase")]
 struct QueryObj{
     name:String,
     age:Option<i8>,//Non mandatory fields  use Option<T>
     hobby:Option<Vec<String>>//Non mandatory fields  use Option<T>
 }

 ```