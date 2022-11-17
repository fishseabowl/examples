
```rust
pub trait DataProvider<Data> {
    async fn get_data(&mut self) -> Option<Data>;
}

pub trait FinalizationHandler<Data> {
    fn data_finalized(&mut self, data: Data);
}
```
