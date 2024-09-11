use std::error::Error;

use tokio::net::{
    TcpListener,
    TcpStream,
};

pub async fn server_bind(databind:String) -> Result<(), Box<dyn Error>>{
    let listner = TcpListener::bind(databind.as_str()).await?;

    while let Ok((inbound,_)) = listner.accept().await{
        tokio::spawn(async move {
            // tcp的连接被移动到该协程中，我们只要专注的处理该stream即可

        });
    }



    Ok(())
}