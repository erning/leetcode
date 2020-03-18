package solution

func twoSum(nums []int, target int) []int {
	m := make(map[int]int, len(nums))
	for i, v := range nums {
		complement := target - v
		if _, ok := m[complement]; ok {
			return []int{m[complement], i}
		}
		m[v] = i
	}
	return nil
}
