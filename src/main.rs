use chess_engine::uci::UciEngine;

fn main() {
    let mut engine = UciEngine::new();
    engine.uci_loop();
}
