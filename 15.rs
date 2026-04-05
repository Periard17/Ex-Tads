use std::collections::VecDeque;

fn sliding_window(nums: Vec<i32>, k: usize) -> Vec<i32> {

    let mut resultado = Vec::new();
    let mut deque: VecDeque<usize> = VecDeque::new();

    for i in 0..nums.len() {

        while let Some(&j) = deque.back() {
            if nums[j] <= nums[i] {
                deque.pop_back();
            } else {
                break;
            }
        }

        deque.push_back(i);

        if deque.front() == Some(&(i-k)) {
            deque.pop_front();
        }

        if i >= k-1 {
            resultado.push(nums[*deque.front().unwrap()]);
        }
    }

    resultado
}
