use fast_hampath::fastpath::HampathBuilder;
use typed_arena::Arena;

fn main() {
    for _ in 0..100{
        let arena = Arena::new();
        let builder = HampathBuilder::new_random(5, &arena);
        let (path, graph) = builder.solution_pair();
        
        println!("Path is {:?}", path);
        println!("{}", graph);
    }
}
