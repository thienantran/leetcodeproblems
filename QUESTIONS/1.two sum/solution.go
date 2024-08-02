func twoSum(nums []int, target int) []int {
    m := make(map[int]int)
    for idx, num := range nums {
        if req, isPresent := m[target - num]; isPresent {
            return []int{req, idx}
        }
        m[num] = idx
    }
    return []int{}
}
