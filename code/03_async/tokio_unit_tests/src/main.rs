fn main() {}

#[allow(dead_code)]
async fn double(n: i32) -> i32 {
    n * 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn will_not_work() {
        // This will not work because we are not in an async context
        // let result = double(2);
        // assert_eq!(result, 4);
    }

    #[test]
    fn the_hard_way() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        assert_eq!(rt.block_on(double(2)), 4);
    }

    #[tokio::test]
    async fn the_easy_way() {
        assert_eq!(double(2).await, 4);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn single_thread_tokio() {
        assert_eq!(double(2).await, 4);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn tamed_multi_thread() {
        assert_eq!(double(2).await, 4);
    }
}
