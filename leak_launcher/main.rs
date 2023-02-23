use std::convert::TryInto;
use fluence_app_service::AppService;
use fluence_app_service::TomlMarineConfig;
use fluence_app_service::AppServiceConfig;
use serde_json::json;

fn main() {

    let config = TomlMarineConfig::load("./Config.toml").unwrap();
    let config = AppServiceConfig {
        service_base_dir: std::path::PathBuf::new(),
        marine_config: config.try_into().unwrap()
    };

    let service_name = "test_service";
    let mut service = AppService::new(config, service_name, <_>::default()).unwrap();

    let db_path = "./tmp/db.sqlite";
    service.call("create", json!(db_path), <_>::default()).unwrap();

    for i in 0..500 {
        if i % 50 == 0 {
            println!("insert_1 {}:\n{}", i, service.module_memory_stats());
        }
        service.call("insert_1", json!(db_path), <_>::default()).unwrap();
    }

    for i in 0..500 {
        if i % 50 == 0 {
            println!("insert_2: {} - {}", i, service.module_memory_stats());
        }
        service.call("insert_2", json!(db_path), <_>::default()).unwrap();
    }

    for i in 0..500 {
        if i % 50 == 0 {
            println!("select_1 {}:\n{}", i, service.module_memory_stats());
        }
        service.call("select_1", json!(db_path), <_>::default()).unwrap();
    }

    for i in 0..500 {
        if i % 50 == 0 {
            println!("select_2 {}:\n{}", i, service.module_memory_stats());
        }
        service.call("select_2", json!(db_path), <_>::default()).unwrap();
    }
}
