//noinspection ALL
#[allow(dead_code)]
pub struct Solution {}

//You are given an array of people, people, which are the attributes of some peo
//ple in a queue (not necessarily in order). Each people[i] = [hi, ki] represents
//the ith person of height hi with exactly ki other people in front who have a hei
//ght greater than or equal to hi.
//
// Reconstruct and return the queue that is represented by the input array peopl
//e. The returned queue should be formatted as an array queue, where queue[j] = [h
//j, kj] is the attributes of the jth person in the queue (queue[0] is the person
//at the front of the queue).
//
//
// Example 1:
//
//
//Input: people = [[7,0],[4,4],[7,1],[5,0],[6,1],[5,2]]
//Output: [[5,0],[7,0],[5,2],[6,1],[4,4],[7,1]]
//Explanation:
//Person 0 has height 5 with no other people taller or the same height in front.
//
//Person 1 has height 7 with no other people taller or the same height in front.
//
//Person 2 has height 5 with two persons taller or the same height in front, whi
//ch is person 0 and 1.
//Person 3 has height 6 with one person taller or the same height in front, whic
//h is person 1.
//Person 4 has height 4 with four people taller or the same height in front, whi
//ch are people 0, 1, 2, and 3.
//Person 5 has height 7 with one person taller or the same height in front, whic
//h is person 1.
//Hence [[5,0],[7,0],[5,2],[6,1],[4,4],[7,1]] is the reconstructed queue.
//
//
// Example 2:
//
//
//Input: people = [[6,0],[5,0],[4,0],[3,2],[2,2],[1,4]]
//Output: [[4,0],[5,0],[2,2],[3,2],[1,4],[6,0]]
//
//
//
// Constraints:
//
//
// 1 <= people.length <= 2000
// 0 <= hi <= 106
// 0 <= ki < people.length
// It is guaranteed that the queue can be reconstructed.
//
// Related Topics Array Greedy Sorting
// 👍 4211 👎 465


//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // reconstruct to ladder form
        let mut people = people;
        people.sort_by(|x, y| {
            y[0].cmp(&x[0]).then(x[1].cmp(&y[1]))
        });

        let mut ret = Vec::with_capacity(people.len());
        for p in people.into_iter() {
            let insert_index = p[1] as usize;
            if insert_index >= ret.len() {
                ret.push(p);
            } else {
                ret.insert(insert_index, p);
            }
        }
        ret
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_reconstruction_by_height() {
        assert_eq!(
            Solution::reconstruct_queue(vec![
                vec![6, 0],
                vec![5, 0],
                vec![4, 0],
                vec![3, 2],
                vec![2, 2],
                vec![1, 4]
            ]),
            vec![
                vec![4, 0],
                vec![5, 0],
                vec![2, 2],
                vec![3, 2],
                vec![1, 4],
                vec![6, 0]
            ]
        );
        assert_eq!(
            Solution::reconstruct_queue(vec![
                vec![7, 0],
                vec![4, 4],
                vec![7, 1],
                vec![5, 0],
                vec![6, 1],
                vec![5, 2]
            ]),
            vec![
                vec![5, 0],
                vec![7, 0],
                vec![5, 2],
                vec![6, 1],
                vec![4, 4],
                vec![7, 1]
            ]
        );
    }
}
