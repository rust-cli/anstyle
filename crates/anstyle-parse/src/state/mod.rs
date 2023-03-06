#[cfg(test)]
mod codegen;
mod definitions;
mod table;

#[cfg(test)]
pub(crate) use definitions::pack;
pub(crate) use definitions::unpack;
pub(crate) use definitions::Action;
pub(crate) use definitions::State;
pub(crate) use table::state_change;
