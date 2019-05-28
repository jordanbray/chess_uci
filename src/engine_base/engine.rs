use super::engine_options::EngineOptions;
use engine::Id;
use super::search::Search;
use chess_uci::timer::timer::Timer;

struct EngineBase<E: Eval, S: Search<E>> {
    options: EngineOptions,
    id: Id,
    searcher: S,
    timer: Timer,
}

impl<E, S, T> EngineBase<E: Eval, S: Search<E>> {
    pub fn new(timer: Timer, id: Id, options: Vec<EngineOption>) {
        
    }
}
