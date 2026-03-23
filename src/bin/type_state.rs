
enum Sync {
    Ready,
    Completed,
    Failed,
    Error,
}

enum Async {
    Ready,
    Running,
    Completed,
    Failed,
    Error,
}
enum Step {
    Sync(Sync),
    Async(Async)
}

fn main() {

}