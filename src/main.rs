use rocksdb::{ColumnFamilyDescriptor, Options, DB, IteratorMode};
use std::{sync::Arc, thread::sleep};
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

mod file;

#[tokio::main]
async fn main() {
    let path = "./data/_path_for_rocksdb_storage_with_cf_";
    let mut cf_opts = Options::default();
    cf_opts.set_max_write_buffer_number(16);
    let cf = ColumnFamilyDescriptor::new("cf1", cf_opts);

    let mut db_opts = Options::default();
    db_opts.create_missing_column_families(true);
    db_opts.create_if_missing(true);
    let db = DB::open_cf_descriptors(&db_opts, path, vec![cf]).unwrap();

    let cf_handle = db.cf_handle("cf1").unwrap();
    db.put_cf(&cf_handle, b"my key", b"my value").unwrap();
    db.put_cf(&cf_handle, b"my key22", b"my value22").unwrap();
    db.put_cf(&cf_handle, b"my key21", b"my value21").unwrap();
    let it = db.iterator_cf(&cf_handle, IteratorMode::Start);
    for item in it {
        let (key, value) = item.unwrap();
        println!("key: {}",String::from_utf8(key.to_vec()).unwrap());
        println!("val: {}",String::from_utf8(value.to_vec()).unwrap());
    }
    match db.get_cf(&cf_handle, b"my key") {
        Ok(Some(value)) => println!("retrieved value {}", String::from_utf8(value).unwrap()),
        Ok(None) => println!("value not found"),
        Err(e) => println!("operational problem encountered: {}", e),
    }
    db.delete_cf(&cf_handle, b"my key").unwrap();
}

async fn batch_put(db: Arc<DB>, n: i32, keys: i32) {
    let data = file::get_file_contents("/Users/yanghengfei/code/rust/hello_rocksdb/data/stream/files/default/logs/k8s/2023/09/05/02/7104645652294008832NN1zOv.parquet").expect("Unable to read file");
    let start = std::time::Instant::now();
    let start_pos = n * keys;
    for i in 0..keys {
        let key = format!("key_{}", i + start_pos);
        db.put(key.as_bytes(), &data).unwrap();
        if i % 1000 == 0 {
            println!("put {}", i);
            println!("put cost: {:?}", start.elapsed());
        }
    }
    db.flush().unwrap();
    println!("put batch[{n}] done");
    println!("put batch[{n}] cost: {:?}", start.elapsed());
}
