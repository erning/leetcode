package solution

func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
	len1, len2 := len(nums1), len(nums2)
	if len1 > len2 {
		// ensure len(nums1) <= len(nums2)
		len1, len2 = len2, len1
		nums1, nums2 = nums2, nums1
	}
	// fmt.Printf("%v, %v\n", nums1, nums2)

	if len2 == 0 {
		// both arrays are empty?
		return 0
	}
	if len2 == 1 {
		// nums2 has only one item
		if len1 == 0 {
			return float64(nums2[0])
		}
		return float64(nums1[0]+nums2[0]) / 2
	}

	a, b, c, d := 0, len1, (len1+1)/2, 0
	for {
		d = (len1+len2+1)/2 - c
		// fmt.Printf("a=%d b=%d c=%d d=%d\n", a, b, c, d)

		if c > 0 && d < len2 && nums1[c-1] > nums2[d] {
			b = c
			c = a + (b-a)/2
			continue
		}
		if c < len1 && d > 0 && nums2[d-1] > nums1[c] {
			a = c
			c = a + (b-a+1)/2
			continue
		}
		break
	}

	max := func(x, y int) int {
		if x > y {
			return x
		}
		return y
	}
	min := func(x, y int) int {
		if x < y {
			return x
		}
		return y
	}

	x, y := func() (int, int) {
		if len1 == 0 {
			return nums2[d-1], nums2[d]
		}
		if c == 0 {
			if d == len2 {
				return nums2[d-1], nums1[c]
			}
			return nums2[d-1], min(nums1[c], nums2[d])
		}
		if c == len1 {
			if d == 0 {
				return nums1[c-1], nums2[d]
			}
			return max(nums1[c-1], nums2[d-1]), nums2[d]
		}
		return max(nums1[c-1], nums2[d-1]), min(nums1[c], nums2[d])
	}()

	if (len1+len2)%2 == 1 {
		// odd
		return float64(x)
	}
	return float64(x+y) / 2
}
