use std::{net::TcpListener, io::Read};
use crate::http::Request;

pub struct Server{
    addr: String,
}

impl Server{
    pub fn new(addr: String) -> Self{
        Self { addr }
    }

    pub fn run(self){
        println!("Listening on {} ", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        
         loop {
            match listener.accept(){
                Ok((mut stream,_)) =>{
                    let mut buffer=[0;1024];
                    match stream.read(&mut buffer){
                        Ok(_) =>{
                            println!("Reques {}",String::from_utf8_lossy(&buffer));
                            
                            match Request::try_from(&buffer[..]){
                                Ok(request)=>{
                                    /*TODO:*/
                                }
                                Err(e)=>println!("Failed to parse a request {}", e),
                            }
                        }
                        Err(e) => println!("Failed to read from a connection {}", e),
                    }
                } 
                Err(e) => println!("Connection failed with err: {}",e),
            }
         }
    }
}