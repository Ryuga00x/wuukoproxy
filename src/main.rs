use wuukoproxy::{Parser,
                arg::{get_bind_data,
                    Cli,
                    }

                }
        ;
fn main(){
    let arga = Cli::parse();
    let (listen_host,port,binded_adrr) = get_bind_data(&arga);
    

}