use engine_base::EngineOptions;
use engine::Id;
use engine_base::Search;

struct EngineBase<T: Search> {
    options: EngineOptions,
    id: Id,
    searcher: T,
}
