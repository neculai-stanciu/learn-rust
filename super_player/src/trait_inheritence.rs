trait Vehicle {
  fn get_price(&self) -> u64;
}

trait Car: Vehicle {
  fn model(&self) -> String;
}

pub trait From<T> {
  fn from(T) -> Self;
}

struct TeslaRoadster {
  model: String,
  release_date: u16,
}

impl TeslaRoadster {
  fn new(model: &str, release_date: u16) -> Self {
    Self {
      model: model.to_string(),
      release_date,
    }
  }
}

impl Car for TeslaRoadster {
  fn model(&self) -> String {
    "Tesla Roadster I".to_string()
  }
}

impl Vehicle for TeslaRoadster {
  fn get_price(&self) -> u64 {
    200_000
  }
}

fn main() {
  let my_roadster = TeslaRoadster::new("Tesla roadster I", 2020);
  println!(
    "{} is priced at ${}",
    my_roadster.model(),
    my_roadster.get_price()
  );
}
