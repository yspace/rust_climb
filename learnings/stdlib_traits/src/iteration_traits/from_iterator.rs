/*

trait FromIterator<A> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = A>;
}

FromIterator types can be created from an iterator, hence the name. FromIterator is most commonly and idiomatically used by calling the collect method on Iterator:

fn collect<B>(self) -> B
where
    B: FromIterator<Self::Item>;

 */

 fn filter_letters(string: &str) -> String {
    string.chars().filter(|c| c.is_alphabetic()).collect()
}

// All the collections in the standard library impl IntoIterator and FromIterator so that makes it easier to convert between them:

use std::collections::{BTreeSet, HashMap, HashSet, LinkedList};

// ### 相当于迭代器是集合性质类型转换的桥梁  所谓的集合性质如 String channel之类也可以 先into到迭代器 然后其他类型再from｜collect迭代器 就可以实现不同类型的转换了！

// String -> HashSet<char>
fn unique_chars(string: &str) -> HashSet<char> {
    string.chars().collect()
}

// Vec<T> -> BTreeSet<T>
fn ordered_unique_items<T: Ord>(vec: Vec<T>) -> BTreeSet<T> {
    vec.into_iter().collect()
}

// HashMap<K, V> -> LinkedList<(K, V)>
fn entry_list<K, V>(map: HashMap<K, V>) -> LinkedList<(K, V)> {
    map.into_iter().collect()
}

// and countless more possible examples