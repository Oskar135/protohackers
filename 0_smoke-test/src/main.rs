use std::{net::TcpListener,str::from_utf8,io::Read, io::Write, io::Result, thread};

fn main() -> Result<()>{

    let addr= "192.168.0.9:5006"; 
    let listener= TcpListener::bind(addr)?; 
    for stream in listener.incoming(){
        let mut stream = stream?;
        let _ :thread::JoinHandle<Result<()>> = thread::spawn(move || {
            loop {
                let mut rx_bytes = [0u8;  256];
                let readbytes = stream.read(&mut rx_bytes[..])?;
                if readbytes == 0 {
                    break Ok(()); 
                    
                }
                let _recieved =from_utf8(&rx_bytes);
                println!("{:?}",_recieved);
                stream.write(&rx_bytes[..readbytes])?;
            }
        });
    }
    Ok(())
}
