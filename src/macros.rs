#[macro_export]
macro_rules! build_data_context {
    ($($data:expr),+) => {
      DataContext::new($(
        $data.prompt().unwrap(),
      )+)
    };
}