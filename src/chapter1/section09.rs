/// 시저 암호
/// 알파벳을 n칸씩 밀어서(shift) 다른 알파벳으로 치환
/// 순환이 끝나는 시점에서는 다시 처음으로 돌아가 치환 한다.

fn encrypt(text: &str, shift: i16) -> String {
    // 'A'와 'Z'의 문자코드를 i16 타입으로 취득
    let code_a = 'A' as i16;
    let code_z = 'Z' as i16;
    // 결과를 대입할 변수를 선언
    let mut result = String::new();
    // 한 글자씩 치환 처리
    for ch in text.chars() {
        // 문자 코드로 변환
        let mut code = ch as i16;
        // A와 Z 사이에 있는 값인가?
        if code >= code_a && code <= code_z {
            // shift만큼 뒤의 문자로 치환
            code = (code - code_a + shift + 26) % 26 + code_a;
        }
        // 문자 코드를 다시 문자로 변환
        result.push((code as u8) as char);
    }
    result
}

fn encrypt_with_closure(text: &str, shift: i16) -> String {
    let a = 'A' as i16;
    let is_az = |c| ('A'..='Z').contains(&c);
    let conv = |c| (((c - a + shift + 26) % 26 + a) as u8) as char;
    let enc1 = |c| if is_az(c) { conv(c as i16) } else { c };
    text.chars().map(enc1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        assert_eq!(encrypt("ABC", 1), "BCD");
        assert_eq!(encrypt("XYZ", 1), "YZA");
        assert_eq!(encrypt("ABC", 26), "ABC");
        assert_eq!(encrypt("ABC", 27), "BCD");
    }

    #[test]
    fn test_encrypt_with_closure() {
        assert_eq!(encrypt_with_closure("ABC", 1), "BCD");
        assert_eq!(encrypt_with_closure("XYZ", 1), "YZA");
        assert_eq!(encrypt_with_closure("ABC", 26), "ABC");
        assert_eq!(encrypt_with_closure("ABC", 27), "BCD");
    }
}
