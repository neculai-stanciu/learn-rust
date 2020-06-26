struct GenericStruct<T>(T);

pub struct Container<T> {
  item: T,
}

impl<T> Container<T> {
  pub fn new(item: T) -> Self {
    Container { item }
  }
}

#[cfg(test)]
mod tests {
  use super::Container;

  #[test]
  fn container_should_work() {
    let container = Container::new("Test");
    println!("Container: {}", container.item);
    assert_eq!(container.item, "Test")
  }
}
