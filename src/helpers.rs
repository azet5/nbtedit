#[macro_export]
macro_rules! is_dir {
    ($self: expr, $t: expr) => {
        if let Some(_) = $self.directory {
            Some($t)
        } else {
            None
        }
    };
}