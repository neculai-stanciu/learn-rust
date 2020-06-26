use std::fmt::Display;

struct Game;
struct Enemy;
struct Hero;

struct Foo<T: Display> {
  bar: T,
}

trait Loadable {
  fn init(&self);
}

impl Loadable for Enemy {
  fn init(&self) {
    println!("Enemy loaded!");
  }
}

impl Loadable for Hero {
  fn init(&self) {
    println!("Hero loaded!");
  }
}

impl Game {
  fn load<T: Loadable>(&self, entity: T) {
    entity.init();
  }
}

fn main() {
  let game = Game;
  game.load(Enemy);
  game.load(Hero);
  let foo = Foo { bar: "Buna ziua" };
  println!("{}", foo.bar);
}
