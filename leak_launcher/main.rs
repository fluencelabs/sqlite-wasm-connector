use fluence_app_service::AppService;
use fluence_app_service::TomlMarineConfig;
use fluence_app_service::AppServiceConfig;
use serde_json::json;
use std::convert::TryInto;

fn main() {
    let config = TomlMarineConfig::load("Config_spell.toml").unwrap();
    let config = AppServiceConfig {
        service_base_dir: std::path::PathBuf::new(),
        marine_config: config.try_into().unwrap()
    };

    let service_name = "spell_service";
    let mut spell_service = AppService::new(config, service_name, <_>::default()).unwrap();

    let config = TomlMarineConfig::load("Config_test.toml").unwrap();
    let config = AppServiceConfig {
        service_base_dir: std::path::PathBuf::new(),
        marine_config: config.try_into().unwrap()
    };

    let service_name = "test_service";
    let mut test_service = AppService::new(config, service_name, <_>::default()).unwrap();

    let db_path = "./tmp/db.sqlite";
    test_service.call("create_4", json!(db_path), <_>::default()).unwrap();
    //test_service.call("set_limit", json!(1*1024*1024), <_>::default()).unwrap();

    /*
    for i in 0..500 {
        if i % 50 == 0 {
            println!("test list_push_string_1 {}:\n{}", i, test_service.module_memory_stats());
        }
        let result = test_service.call("list_push_string_1", json!(["asdasd", "sadsad"]), <_>::default()).unwrap();
    }
     */
    for i in 0..500 {
        if i % 50 == 0 {
            println!("test insert_5 {}:\n{}", i, test_service.module_memory_stats());
        }
        let result = test_service.call("insert_5", json!(["asdasd", "sadsad"]), <_>::default()).unwrap();
    }

    for i in 0..500 {
        if i % 50 == 0 {
            println!("test list_push_string_6 {}:\n{}", i, test_service.module_memory_stats());
        }
        let result = test_service.call("list_push_string_6", json!(["asdasd", "sadsad"]), <_>::default()).unwrap();
    }

    for i in 0..500 {
        if i % 50 == 0 {
            println!("test list_push_string_5 {}:\n{}", i, test_service.module_memory_stats());
        }
        let unique_key = create_unique_key(500 + i);
        let result = test_service.call("list_push_string_5", json!([unique_key, "sadsad"]), <_>::default()).unwrap();
    }

    for i in 0..500 {
        if i % 50 == 0 {
            println!("get_all_errors {}:\n{}", i, spell_service.module_memory_stats());
        }
        let result = spell_service.call("insert_2", json!([]), <_>::default()).unwrap();
    }

    for i in 0..500 {
        if i % 50 == 0 {
            println!("select_1 {}:\n{}", i, spell_service.module_memory_stats());
        }
        let result = spell_service.call("select_1", json!([]), <_>::default()).unwrap();
    }

    for i in 0..500 {
        if i % 50 == 0 {
            println!("select_5 {}:\n{}", i, spell_service.module_memory_stats());
        }
        let result = spell_service.call("select_5", json!([]), <_>::default()).unwrap();
    }
}

fn create_unique_key(index: i32) -> String {
    let mut first = String::new();

    for _ in 0..(index+1) {
        first += "a";
    }

    first
}
