use fast_hampath::fastpath_naive::HampathBuilder;
use typed_arena::Arena;

fn main() {
    for _ in 0..100{
        let arena = Arena::new();
        let mut builder = HampathBuilder::new_random(5, &arena);
        let path = builder.solve_path();
        let graph = builder.into_graph();

        println!("Path is {:?}", path);
        println!("{}", graph);
    }
}
