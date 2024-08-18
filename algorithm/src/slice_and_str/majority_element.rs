/*
  169. 多数元素

给定一个大小为 n 的数组 nums ，返回其中的多数元素。多数元素是指在数组中出现次数 大于 ⌊ n/2 ⌋ 的元素。

你可以假设数组是非空的，并且给定的数组总是存在多数元素。

 

示例 1：

输入：nums = [3,2,3]
输出：3
示例 2：

输入：nums = [2,2,1,1,1,2,2]
输出：2
 

提示：
n == nums.length
1 <= n <= 5 * 104
-109 <= nums[i] <= 109
 

进阶：尝试设计时间复杂度为 O(n)、空间复杂度为 O(1) 的算法解决此问题。
*/

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut res = nums[0];
    let mut count = 1;

    for i in 1..nums.len() {
        if count == 0 {
            res = nums[i];
            count += 1;
        } else if nums[i] == res {
            count += 1;
        } else {
            count -= 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority_element() {
        let nums = vec![2,2,1,1,1,2,2];
        assert_eq!(majority_element(nums), 2)
    }
}