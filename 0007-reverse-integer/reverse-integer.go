package solution

func reverse(x int) int {
	var b int
	var max int
	if x < 0 {
		x = -x
		b = -1
		max = 0x7fffffff + 1
	} else {
		b = 1
		max = 0x7fffffff
	}
	var y int
	for x > 0 {
		m := x % 10
		x = x / 10
		if y > (max-m)/10 {
			return 0
		}
		y = y*10 + m
	}
	return y * b
}

func reverse_faster(x int) int {
	x = int(int32(x))
	var b int
	if x < 0 {
		x = -x
		b = -1
	} else {
		b = 1
	}
	var y int
	for x > 0 {
		y = y*10 + x%10
		x /= 10
	}
	y = y * b
	if y < -1<<31 || y > (1<<31)-1 {
		return 0
	}
	return y
}
