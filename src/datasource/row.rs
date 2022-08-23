pub enum Row<T> {
    Generic(T),
    Decryptable(T),
    NonDecryptable(T),
}
