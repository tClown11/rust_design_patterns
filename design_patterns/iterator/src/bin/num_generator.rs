//! 基于 Iterator 实现数字生成器
use std::{collections::{HashMap, HashSet}, ops::AddAssign};

pub struct NumberGenerator {
    // 开始
    begin: usize,
    // 结束
    end: usize,
}

impl NumberGenerator {
    pub fn new(begin: usize, end: usize) -> Self {
        Self { begin, end }
    }
}

impl Iterator for NumberGenerator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.begin < self.end {
            let res = self.begin;
            // begin += 1
            self.begin.add_assign(1);
            Some(res)
        } else {
            None
        }
    }
}

// 统计迭代器元素个数
fn count_iterator(mut iterator: impl Iterator) -> usize {
    let mut res = 0;
    while let Some(_) = iterator.next() {
        res.add_assign(1);
    }
    res
}

fn main() {
    let nums = NumberGenerator::new(90, 100);
    for i in nums {
        print!("item is: {i} \n");
    }

    // 常见的 iterator 上的方法
    // filter for_each
    (0..5) // 初始的迭代器
        .filter(|n| n%2 == 0) // 过滤数据
        .for_each(|n| println!("{n}")); // 对迭代器每个元素进行操作

    // map
    (0..5)
        .map(|n| n as f64)
        .map(|n| n.sqrt())
        .for_each(|n| println!("{n}"));

    // 统计元素个数
    let a = count_iterator(0..=500);
    println!("collection count: {a}"); // 输出 501

    // 求和
    println!("sum: {}", (1..=100).sum::<usize>()); // 输出 5050

    // fold
    let sum = (1..=100).fold(1, |init, n| init + n);
    println!("sum: {sum}");  // 输出 5051

    // collect map 只要有 None 值，整个 iterator 都是 None
    let vec = (0..5).collect::<Vec<usize>>();
    assert_eq!(vec, vec![0, 1, 2, 3, 4]);
    let res = (1..50)
        .map(|n| if n <= 3 {Some(n)} else {None})
        .collect::<Option<Vec<usize>>>();
    assert_eq!(res, None);

    // collect hashset 
    (0..5)
        .map(|n| if n > 3 { 3 } else { n })
        .collect::<HashSet<usize>>()
        .iter()
        .for_each(|n| println!("{n}")); // 输出 0 1 2 3 顺序不定

    // flatten 扁平化处理
    vec![Some(1), None, Some(2)]
        .into_iter()
        .flatten()
        .collect::<HashSet<usize>>()
        .into_iter()
        .for_each(|n| println!("for_each:{n}"));
}