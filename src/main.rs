use project_shmove::engine;

mod game;

fn main() {
  let scene = game::GameScene::new();
  pollster::block_on(engine::run(scene));
}
