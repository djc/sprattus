use futures::{
    future::Future,
    stream::Stream
};

mod models;
use crate::models::*;

fn main() {
    let mut runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
    let mut client : Client = runtime.block_on(get_client()).unwrap();
    let client_ref = Arc::new(Mutex::new(client));
    let products = runtime.block_on(get_products(client_ref));
    dbg!(products);
}

fn get_products(client_ref: Arc<Mutex<Client>>) -> Box<dyn Future<Item=Vec<Product>, Error=error::Error> + Send> {
    let fut = client_ref.lock().unwrap().prepare("SELECT * FROM products ORDER BY prod_id DESC limit 5");
    let ret_val = fut
        .and_then(move |statement|{
            client_ref.lock().unwrap().query(&statement, &[]).collect()
        })
        .map(move |rows|{
            let mut n : usize = 0;
            rows.iter().map(move |row|{
                Product {
                    id: 1,//row.get(0),
                    category: row.get(1),
                    title: row.get(2),
                    actor: row.get(3),
                    price: BigDecimal::from(0),
                    special: row.get(5),
                    common_prod_id: row.get(6)
                }
            }).collect()
        });
    Box::new(ret_val)
}

fn get_client() -> Box<dyn Future<Item = Client, Error = error::Error> + Send> {
    Box::new(tokio_postgres::connect("postgresql://localhost/dellstore2?user=tg", NoTls)
        .map(|(client, connection)| {
            // The connection object performs the actual communication with the database,
            // so spawn it off to run on its own.
            let connection = connection.map_err(|e| eprintln!("connection error: {}", e));
            tokio::spawn(connection);
            client
        }))
}