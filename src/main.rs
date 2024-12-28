mod scalar_types;
mod compound_types;
mod collections;
mod btree;
mod channels;
mod mutex_rwlock;
mod rc_arc;

fn main() {
    scalar_types::print_scalar_types();
    compound_types::print_compound_types();
    collections::print_collections();
    btree::btree_example();
    channels::channel_example();
    mutex_rwlock::mutex_example();
    mutex_rwlock::rwlock_example();
    rc_arc::rc_example();
    rc_arc::arc_example();
}