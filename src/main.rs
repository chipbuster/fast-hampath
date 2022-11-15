use fast_hampath::slowpath::HampathBuilder;
use typed_arena::Arena;
use fast_hampath::tngraph::TournamentGraph;

fn main() {
    for _ in 0..100{
        let arena = Arena::new();
        let graph = TournamentGraph::new_random(100, &arena);
        let mut builder = HampathBuilder::new(&graph);

        let path = builder.solve();
    }
}