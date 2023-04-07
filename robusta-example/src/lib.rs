use robusta_jni::bridge;

use std::sync::RwLock;

use once_cell::sync::Lazy;
use rdkafka::producer::{DefaultProducerContext, ThreadedProducer};
use std::collections::HashMap;

static GLOBAL_DATA: Lazy<RwLock<HashMap<String, ThreadedProducer<DefaultProducerContext>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

#[bridge]
mod jni {
    use std::time::Duration;
    use crate::GLOBAL_DATA;
    use rdkafka::producer::{BaseRecord, DefaultProducerContext, ThreadedProducer};
    use rdkafka::ClientConfig;
    use rdkafka::config::RDKafkaLogLevel;
    use robusta_jni::convert::{
        Field, IntoJavaValue, Signature, TryFromJavaValue, TryIntoJavaValue,
    };
    use robusta_jni::jni::errors::Error as JniError;
    use robusta_jni::jni::errors::Result as JniResult;
    use robusta_jni::jni::objects::AutoLocal;
    use robusta_jni::jni::JNIEnv;
    use simple_logger::SimpleLogger;
    use log::{info, LevelFilter};

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(com.example.robusta)]
    pub struct HelloWorld<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
        #[field]
        foo: Field<'env, 'borrow, String>,
    }

    impl<'env: 'borrow, 'borrow> HelloWorld<'env, 'borrow> {
        #[constructor]
        pub extern "java" fn new(env: &'borrow JNIEnv<'env>) -> JniResult<Self> {}

        pub extern "jni" fn special(mut input1: Vec<i32>, input2: i32) -> Vec<String> {
            input1.push(input2);
            input1.iter().map(ToString::to_string).collect()
        }

        pub extern "jni" fn nativeFun(self, env: &JNIEnv, static_call: bool) -> JniResult<i32> {
            if static_call {
                Ok(HelloWorld::staticJavaAdd(env, 1, 2))
            } else {
                let a = self.javaAdd(env, 0, 0)?;
                Ok(a + self.javaAdd(env, 1, 2)?)
            }
        }

        #[call_type(safe(
            exception_class = "java.lang.IllegalArgumentException",
            message = "something bad happened"
        ))]
        pub extern "jni" fn catchMe(self, _env: &JNIEnv) -> JniResult<i32> {
            Err(JniError::NullPtr("catch me if you can"))
        }

        pub extern "java" fn javaAdd(&self, _env: &JNIEnv, i: i32, u: i32) -> JniResult<i32> {}

        #[call_type(unchecked)]
        pub extern "java" fn staticJavaAdd(env: &JNIEnv, i: i32, u: i32) -> i32 {}

        pub extern "jni" fn setStringHelloWorld(mut self) -> JniResult<()> {
            println!("[rust]: self.foo: \"{}\"", self.foo.get()?);
            self.foo.set("hello world".into())?;
            Ok(())
        }
    }

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(org.apache.kafka.clients.producer)]
    pub struct KafkaProducer<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
    }

    impl<'env: 'borrow, 'borrow> KafkaProducer<'env, 'borrow> {
        #[constructor]
        pub extern "java" fn new(env: &'borrow JNIEnv<'env>) -> JniResult<Self> {}

        pub extern "jni" fn init(self, bootstrap_servers: String, use_ssl: bool) -> JniResult<()> {
            SimpleLogger::new()
                .with_level(LevelFilter::Info)
                .init()
                .unwrap();

            let mut client_config = ClientConfig::new();

            // client_config.set("compression.type", "lz4");

            info!("bootstrap.servers {}", bootstrap_servers);
            client_config.set("bootstrap.servers", bootstrap_servers.clone());
            if use_ssl {
                client_config.set("security.protocol", "SSL");
            }

            client_config.set_log_level(RDKafkaLogLevel::Debug);

            let producer: ThreadedProducer<DefaultProducerContext> =
                client_config.create().expect("Producer creation failed");

            let mut map = GLOBAL_DATA.write().unwrap();

            map.insert(bootstrap_servers.clone(), producer);

            println!("Created producer {bootstrap_servers} {use_ssl}");

            Ok(())
        }

        pub extern "jni" fn send(
            self,
            bootstrap_servers: String,
            topic: String,
            key: String,
            payload: String,
        ) -> JniResult<()> {
            let map = GLOBAL_DATA.read().unwrap();

            let producer = map.get(&bootstrap_servers).unwrap();

            let result = producer.send(
                BaseRecord::with_opaque_to(&topic, ())
                    .key(&key)
                    .payload(&payload),
            );

            // producer.poll(Duration::from_millis(100));

            println!("Send result {result:?}");

            Ok(())
        }

        pub extern "jni" fn close(self, bootstrap_servers: String) -> JniResult<()> {
            let mut map = GLOBAL_DATA.write().unwrap();

            map.remove(&bootstrap_servers);

            println!("Closed producer {bootstrap_servers}");

            Ok(())
        }
    }
}
