use serde:: {Deserialize, Serialize};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream}
};

#[derive(Serialize, Debug)]
struct Response {
    method: &'static str,
    prime: bool
}

#[derive(Deserialize, Debug)]
struct Message {
    method: String,
    number: i64
}

impl Message {
    fn is_ok(&self) -> bool {
        if self.method == "isPrime"{
            return true;
        }
        return false; 
    }

    fn is_prime(&self) -> bool {
        if self.number <= 1{
            return false; 
        }
        for i in  2..self.number {
            if self.number % i ==0 {
                let res = self.number % i; 
                println!("{res}"); 
                return false; 
            }
        }
        return true; 
    }

    fn get_variables(&self){
        println!("Method is: {0}, Number is: {1} ", self.method, self.number);
    }
    
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    
    let addr= "192.168.0.9:5006"; 
    let listener = TcpListener::bind(addr).await?; 

    loop {
        let (mut socket, _) = listener.accept().await?; 

        tokio::spawn( async move {
            handle_connection(&mut socket).await
        }); 

    }
   // 
   // let addr= "192.168.0.9:5006"; 
   // let listener= TcpListener::bind(addr)?; 
   // for stream in listener.incoming(){
   //     let mut stream = stream?; 
   //     let _ :thread::JoinHandle<Result<()>> = thread::spawn(move || {
   //         loop {
   //             let reader = BufReader::new(stream);
   //             for rad in reader.lines(){
   //                 let rad =rad?;
   //                 let mut m = serde_json::Deserializer::from_str(&rad);
   //                 let msg = Message::deserialize(&mut m)?;
   //             }

   //         }
   //     });
   // }
   // Ok(())
}

async fn handle_connection(socket: &mut TcpStream){       

    let (reader, mut writer) = socket.split(); 
    let mut reader = BufReader::new(reader); 
    let mut buf = String::new(); 
    
    while reader.read_line(&mut buf).await.expect("Failed to read line from socket!")!=0 {
        let mut m = serde_json::Deserializer::from_str(&buf); 
        let msg= Message::deserialize(&mut m); 
        match msg {
            Ok(msg) =>{
                msg.get_variables(); 
                if msg.is_ok(){
                    let resp = Response {
                        method: "isPrime",
                        prime: msg.is_prime()
                    }; 
                    let mut resp = serde_json::to_vec(&resp).expect("Failed to serialize response!"); 
                    resp.push(b'\n');
                    writer.write_all(&resp).await.expect("Failed to respond!"); 
                    buf.clear();

                }
            },
            _ => {println!("Error while reading from socket!")} 
        }
        //TODO : make vec response and add b\n'
    }
    return; 

}