use crate::tree::FileTree;
use std::rc::Rc;

pub fn prune_invalid_nodes<F>(tree: &mut FileTree, under: &str, criterion: F)
where
    F: Fn(&str) -> bool + 'static,
{
    let criterion_rc = Rc::new(criterion);
    _prune_invalid_nodes_impl(tree, under, criterion_rc);
}

fn _prune_invalid_nodes_impl(
    tree: &mut FileTree,
    under: &str,
    criterion: Rc<dyn Fn(&str) -> bool>,
) {
    if tree
        .children
        .iter()
        .any(|c| c.file_name_lossy().is_some_and(|f| f.contains(under)))
    {
        _prune_invalid_nodes(tree, &criterion);
    } else if !tree.children.is_empty() {
        tree.children
            .iter_mut()
            .for_each(|c| _prune_invalid_nodes_impl(c, under, criterion.clone()));
    }
}

fn _prune_invalid_nodes(tree: &mut FileTree, criterion: &Rc<dyn Fn(&str) -> bool>) {
    tree.children
        .iter()
        .enumerate()
        .filter_map(|(i, t)| {
            t.children
                .iter()
                .all(|c| c.file_name_lossy().is_some_and(|name| criterion(&name)))
                .then_some(i)
        })
        .collect::<Vec<usize>>()
        .iter()
        .rev()
        .for_each(|&i| _ = tree.children.remove(i));
}
