/*
 * Copyright 2021 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

mod tests {
    use marine_rs_sdk_test::marine_test;

    #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts/")]
    fn test1(test: marine_test_env::test::ModuleInterface) {
        test.test1()
    }

    #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts/")]
    fn test2(test: marine_test_env::test::ModuleInterface) {
        test.test2()
    }

    #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts/")]
    fn test3(test: marine_test_env::test::ModuleInterface) {
        test.test3()
    }

    #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts/")]
    fn test4(test: marine_test_env::test::ModuleInterface) {
        test.test4()
    }

    #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts/")]
    fn test5(test: marine_test_env::test::ModuleInterface) {
        test.test5()
    }

    #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts/")]
    fn test6(test: marine_test_env::test::ModuleInterface) {
        test.test6()
    }

    #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts/")]
    fn test7(test: marine_test_env::test::ModuleInterface) {
        test.test7()
    }

    #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts/")]
    fn test8(test: marine_test_env::test::ModuleInterface) {
        test.test8()
    }

    #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts/")]
    fn test9(test: marine_test_env::test::ModuleInterface) {
        test.test9()
    }

    #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts/")]
    fn test10(test: marine_test_env::test::ModuleInterface) {
        test.test10()
    }

    #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts/")]
    fn test11(test: marine_test_env::test::ModuleInterface) {
        test.test11()
    }

    #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts/")]
    fn test12(test: marine_test_env::test::ModuleInterface) {
        test.test12()
    }
}
