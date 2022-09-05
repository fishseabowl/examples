// Enumeration of states
enum Mytask {
    // Enumeration of all states
    Init,              // Initial task state
    AwaitSleep(Sleep), // Task is waiting for `sleep` to complete
}

fn poll(&mut self, cx: &mut Context) -> Poll<()> {
    match self {
        Init => {
            // Initial state
            let sleep = time::sleep(ms_10); // Register timeout
            sleep.poll(cx); // Advance 'sleep' state machine
            self = AwaitSleep(sleep);
            return Poll::Pending;
        }
        AwaitSleep(sleep) => {
            return sleep.poll(cx);
        }
    }
}
