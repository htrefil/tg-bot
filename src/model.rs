use rand::distributions::{Distribution, WeightedIndex};
use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct Model<T> {
    nodes: Vec<Node<T>>,
}

impl<T> Model<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new(items: impl IntoIterator<Item = T>) -> Model<T> {
        let mut map = HashMap::<T, HashMap<T, u32>>::new();
        let mut last = None;

        for item in items {
            if let Some(last) = last.take() {
                let children = map.get_mut(&last).unwrap();
                if let Some(n) = children.get_mut(&item) {
                    *n += 1;
                } else {
                    children.insert(item.clone(), 1);
                }
            }

            if map.get(&item).is_none() {
                map.insert(item.clone(), HashMap::new());
            }

            last = Some(item);
        }

        Model {
            nodes: map
                .into_iter()
                .map(|(item, children)| Node {
                    item: item,
                    weights: children.values().cloned().collect(),
                    children: children.into_iter().map(|(item, _)| item).collect(),
                })
                .collect(),
        }
    }
}

pub struct Node<T> {
    item: T,
    children: Vec<T>,
    weights: Vec<u32>,
}

pub struct Generator<'m, 'r, T, R> {
    model: &'m Model<T>,
    map: HashMap<&'m T, &'m Node<T>>,
    used: HashSet<&'m T>,
    last: Option<&'m Node<T>>,
    rng: &'r mut R,
}

impl<'m, 'r, T, R> Generator<'m, 'r, T, R>
where
    T: Hash + Eq,
{
    pub fn new(model: &'m Model<T>, rng: &'r mut R) -> Generator<'m, 'r, T, R> {
        Generator {
            model: model,
            map: model.nodes.iter().map(|node| (&node.item, node)).collect(),
            used: HashSet::new(),
            last: None,
            rng: rng,
        }
    }
}

impl<'m, 'r, T, R> Iterator for Generator<'m, 'r, T, R>
where
    T: Hash + Eq,
    R: Rng,
{
    type Item = &'m T;

    fn next(&mut self) -> Option<&'m T> {
        loop {
            let node = self.last.take().unwrap_or_else(|| {
                &self.model.nodes[self.rng.gen_range(0, self.model.nodes.len())]
            });
            let item = if node.children.is_empty() {
                &node.item
            } else {
                let idx = WeightedIndex::new(&node.weights)
                    .unwrap()
                    .sample(&mut self.rng);
                &node.children[idx]
            };

            if self.used.contains(&item) {
                self.used.clear();
                continue;
            }

            self.used.insert(&item);
            self.last = Some(self.map.get(&item).unwrap());

            return Some(item);
        }
    }
}
