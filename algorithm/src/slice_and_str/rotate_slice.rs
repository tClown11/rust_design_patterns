/*
    189. 轮转数组

给定一个整数数组 nums，将数组中的元素向右轮转 k 个位置，其中 k 是非负数。

 

示例 1:

输入: nums = [1,2,3,4,5,6,7], k = 3
输出: [5,6,7,1,2,3,4]
解释:
向右轮转 1 步: [7,1,2,3,4,5,6]
向右轮转 2 步: [6,7,1,2,3,4,5]
向右轮转 3 步: [5,6,7,1,2,3,4]
示例 2:

输入：nums = [-1,-100,3,99], k = 2
输出：[3,99,-1,-100]
解释: 
向右轮转 1 步: [99,-1,-100,3]
向右轮转 2 步: [3,99,-1,-100]
 

提示：

1 <= nums.length <= 105
-231 <= nums[i] <= 231 - 1
0 <= k <= 105
 

进阶：

尽可能想出更多的解决方案，至少有 三种 不同的方法可以解决这个问题。
你可以使用空间复杂度为 O(1) 的 原地 算法解决这个问题吗？
*/

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let mut k = k as usize;
    let l = nums.len();

    k %= l;
    let k1 = l-k;

    if k != 0 {
        // l-k 长度的前半和后半反转
        for i in 0..(k1-1)/2 + 1 {
            nums.swap(i, k1-i-1);
        }

        // k 长度的前半和后半反转
        for i in k1..(k-1)/2 + k1 + 1 {
            nums.swap(i, l-i-1+k1);
        }
        // 数组整体反转
        nums.reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut nums = vec![1,2,3,4,5,6,7];
        rotate(&mut nums, 3);
        assert_eq!(nums, vec![5,6,7,1,2,3,4])
    }
}