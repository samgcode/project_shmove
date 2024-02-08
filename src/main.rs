use project_shmove::engine;

fn main() {
  pollster::block_on(engine::run());
}
