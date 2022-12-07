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
impl Response    {

    fn get_variables(&self){
        println!("Method is: {0}, Prime: {1} ", self.method, self.prime);
    }
}

#[derive(Deserialize, Debug)]
struct Message {
    method: String,
    number: f64
}

impl Message {
    fn is_ok(&self) -> bool {
        if self.method == "isPrime"{
            return true;
        }
        return false; 
    }

    fn is_prime(&self) -> bool {
        if self.number <=1.0 || self.number.fract()!=0.0 {
            return false; 
        }

        let num = self.number as u64; 

        let panics=  std::panic::catch_unwind(||slow_primes::is_prime_miller_rabin(num)).is_err();
        if panics{
            return false; 
        }

        return slow_primes::is_prime_miller_rabin(num); 
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
                else{
                    println!("Msg is not OK!"); 
                    writer.write_all(b"malformed \n").await.unwrap(); 
                    break; 
                }
            },
            _  => {
                println!("Error while reading from socket!"); 
                break; 
            } 
        }
    }
    socket.shutdown().await.unwrap();    

}