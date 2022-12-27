To use the AlephBFT crate, the following trait implementations are required:
1. Data: Data type that we want to order
```rust
pub trait Data: 'static + Eq + Clone + Send + Sync + Debug + Hash + Codec { }
```
2. DataProvider: The source of data items that consensus should order
```rust
pub trait DataProvider<Data>: 'static + Sync + Send {
    fn get_data<'life0, 'async_trait>(
        &'life0 mut self
    ) -> Pin<Box<dyn Future<Output = Option<Data>> + Send + 'async_trait, Global>>
    where
        'life0: 'async_trait,
        Self: 'async_trait;
}
```
3. FinalizationHandler: The source of finalization of the units that consensus produces
```rust
pub trait FinalizationHandler<Data>: 'static + Sync + Send {
    fn data_finalized(&mut self, data: Data);
}
```
4. Network: Network represents an interface for sending and receiving NetworkData
```rust
pub trait Network<D>: Send {
    fn send(&self, data: D, recipient: Recipient);
    fn next_event<'life0, 'async_trait>(
        &'life0 mut self
    ) -> Pin<Box<dyn Future<Output = Option<D>> + Send + 'async_trait, Global>>
    where
        'life0: 'async_trait,
        Self: 'async_trait;
}
```
5. Keychain: Abstraction of the signing data and verifying signatures.
```rust
pub trait Keychain: 'static + Index + Clone + Send + Sync {
    type Signature: Signature;

    fn node_count(&self) -> NodeCount;
    fn sign<'life0, 'life1, 'async_trait>(
        &'life0 self,
        msg: &'life1 [u8]
    ) -> Pin<Box<dyn Future<Output = Self::Signature> + Send + 'async_trait, Global>>
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait;
    fn verify(&self, msg: &[u8], sgn: &Self::Signature, index: NodeIndex) -> bool;
}
```
