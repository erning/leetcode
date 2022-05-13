pub fn number_to_words(num: i32) -> String {
    fn f20(num: i32) -> String {
        assert!(num < 20);
        const MAP: [&str; 20] = [
            "Zero",
            "One",
            "Two",
            "Three",
            "Four",
            "Five",
            "Six",
            "Seven",
            "Eight",
            "Nine",
            "Ten",
            "Eleven",
            "Twelve",
            "Thirteen",
            "Fourteen",
            "Fifteen",
            "Sixteen",
            "Seventeen",
            "Eighteen",
            "Nineteen",
        ];
        MAP[num as usize].to_string()
    }

    fn f100(num: i32) -> String {
        assert!(num < 100);
        if num < 20 {
            return f20(num);
        }
        const MAP: [&str; 8] = [
            "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
        ];
        let a = num / 10;
        let b = num % 10;
        let mut s = MAP[(a - 2) as usize].to_string();
        if b != 0 {
            s.push(' ');
            s.push_str(f20(b).as_str());
        }
        s
    }

    fn f1000(num: i32) -> String {
        assert!(num < 1000);
        if num < 100 {
            return f100(num);
        }
        let a = num / 100;
        let b = num % 100;
        let mut s = f20(a);
        s.push_str(" Hundred");
        if b != 0 {
            s.push(' ');
            s.push_str(f100(b).as_str());
        }
        s
    }

    if num == 0 {
        return f20(num);
    }
    let mut n = num;
    let mut i = 0;
    let mut answer = Vec::new();
    loop {
        const MAP: [&str; 3] = ["Thousand", "Million", "Billion"];
        let mut a = Vec::new();
        let b = n % 1000;
        if b != 0 {
            a.push(f1000(b));
        }
        if !a.is_empty() {
            if i > 0 {
                a.push(MAP[i - 1].to_string());
            }
            answer.push(a.join(" "));
        }
        n /= 1000;
        if n == 0 {
            break;
        }
        i += 1;
    }
    answer.reverse();
    answer.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(number_to_words(123), "One Hundred Twenty Three");
        assert_eq!(
            number_to_words(12345),
            "Twelve Thousand Three Hundred Forty Five"
        );
        assert_eq!(
            number_to_words(1234567),
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
        );

        assert_eq!(
            number_to_words(2147483647),
            "Two Billion One Hundred Forty Seven Million Four Hundred Eighty Three Thousand Six Hundred Forty Seven"
        );
        assert_eq!(
            number_to_words(2047083047),
            "Two Billion Forty Seven Million Eighty Three Thousand Forty Seven"
        );

        assert_eq!(number_to_words(1_000), "One Thousand");
        assert_eq!(number_to_words(1_000_000), "One Million");
        assert_eq!(number_to_words(1_000_000_000), "One Billion");
    }
}
