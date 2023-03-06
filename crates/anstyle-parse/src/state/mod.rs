#[cfg(test)]
mod codegen;
mod definitions;
mod table;

#[cfg(test)]
pub(crate) use definitions::pack;
pub(crate) use definitions::unpack;
pub(crate) use definitions::Action;
pub(crate) use definitions::State;

#[inline]
pub(crate) const fn state_change(state: State, byte: u8) -> (State, Action) {
    // Handle state changes in the anywhere state before evaluating changes
    // for current state.
    let mut change = state_change_(State::Anywhere, byte);
    if change == 0 {
        change = state_change_(state, byte);
    }

    // Unpack into a state and action
    unpack(change)
}

#[inline]
const fn state_change_(state: State, byte: u8) -> u8 {
    let state_idx = state as usize;
    let byte_idx = byte as usize;

    table::STATE_CHANGES[state_idx][byte_idx]
}
