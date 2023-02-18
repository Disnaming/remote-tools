#[macro_export]
macro_rules! test {
    ($name:ident, $test:expr) => {
        #[test]
        fn $name() {
            let process = tokio::spawn(async move {
                start_server("0.0.0.0", "8888", "disna-m.top").await;
            });
            $test();
            process.abort();
        }
    };
}