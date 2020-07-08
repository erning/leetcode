package solution

func minInteger(num string, k int) string {
	return minIntegerTenDigits(num, k)
}

//

type Item struct {
	v byte
	p int
}

type Items []*Item

func minIntegerTenDigits(num string, k int) string {
	length := len(num)
	items := make(Items, length)
	var digits [10][]*Item
	for i := 0; i < length; i++ {
		item := &Item{num[i], i}
		items[i] = item
		v := item.v - '0'
		digits[v] = append(digits[v], item)
	}

	for b := 0; b < length && k > 0; b++ {
		item := func() *Item {
			for v := 0; v < 10; v++ {
				if len(digits[v]) > 0 {
					item := digits[v][0]
					if item.p-b <= k {
						digits[v] = digits[v][1:]
						return item
					}
				}
			}
			return nil
		}()
		if item.p == b {
			continue
		}

		v := items[item.p]
		copy(items[b+1:], items[b:item.p])
		items[b] = v
		k -= item.p - b
		for j := b; j < item.p; j++ {
			items[j].p++
		}
	}

	buf := make([]byte, length)
	for i := 0; i < length; i++ {
		buf[i] = items[i].v
	}
	return string(buf)
}

//

func minIntegerSimple(num string, k int) string {
	numBytes := []byte(num)
	length := len(numBytes)

	for b := 0; b < length && k > 0; b++ {
		i := func() int {
			if numBytes[b] == '0' {
				return b
			}
			minV := numBytes[b]
			minI := b
			for i := b + 1; i < length && i <= b+k; i++ {
				if numBytes[i] < minV {
					minV = numBytes[i]
					minI = i
				}
				if minV == '0' {
					break
				}
			}
			return minI
		}()
		if i == b {
			continue
		}

		v := numBytes[i]
		copy(numBytes[b+1:], numBytes[b:i])
		numBytes[b] = v
		k -= i - b
	}
	return string(numBytes)
}
