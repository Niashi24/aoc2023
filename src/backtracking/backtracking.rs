pub trait Action<T> {
    fn execute(&self, state: T) -> T;
    fn undo(&self, state: T) -> T;
}

pub fn backtracking<S, A, FN, IN, FS, FV>(start: S, mut successors: FN, mut success: FS, mut valid_action: FV) -> Option<S>
    where
        A: Action<S>,
        FN: FnMut(&S) -> IN,
        IN: IntoIterator<Item=A>,
        FS: FnMut(&S) -> bool,
        FV: FnMut(&S, &A) -> bool,
{
    step(start, &mut successors, &mut success, &mut valid_action).ok()
}

fn step<S, A, FN, IN, FS, FV>(mut state: S, successors: &mut FN, success: &mut FS, valid_action: &mut FV) -> Result<S, S>
    where
        A: Action<S>,
        FN: FnMut(&S) -> IN,
        IN: IntoIterator<Item=A>,
        FS: FnMut(&S) -> bool,
        FV: FnMut(&S, &A) -> bool,
{
    for action in successors(&state) {
        if !valid_action(&state, &action) {
            continue;
        }

        state = action.execute(state);
        if success(&state) {
            return Ok(state);
        }

        match step(state, successors, success, valid_action) {
            Err(s) => {
                state = action.undo(s);
                continue;
            }
            Ok(x) => { return Ok(x); }
        }
    }

    Err(state)
}

pub fn backtracking_iterative<S, A, FN, IN, FS, FV>(start: S, mut successors: FN, mut success: FS, mut valid_action: FV) -> Option<S>
    where
        A: Action<S>,
        FN: FnMut(&S) -> IN,
        IN: IntoIterator<Item=A>,
        FS: FnMut(&S) -> bool,
        FV: FnMut(&S, &A) -> bool,
{
    let mut actions_to_apply_stack = vec![successors(&start).into_iter()];
    let mut applied_action_stack: Vec<A> = vec![];
    let mut state = start;
    while let Some(actions) = actions_to_apply_stack.last_mut() {
        let Some(a) = actions.next() else {
            state = applied_action_stack.pop()?.undo(state);
            actions_to_apply_stack.pop();
            continue;
        };

        if !valid_action(&state, &a) {
            continue;
        }

        state = a.execute(state);
        if success(&state) {
            return Some(state);
        }

        applied_action_stack.push(a);
        actions_to_apply_stack.push(successors(&state).into_iter());
    }

    None
}